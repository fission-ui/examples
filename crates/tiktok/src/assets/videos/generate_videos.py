#!/usr/bin/env python3
"""
Generate minimal valid MP4 test video files with colored frames.
Uses raw byte construction to create valid H.264+AAC MP4 files
without any external dependencies beyond the Python standard library.

Each video has a single solid-color I-frame encoded as minimal H.264 baseline,
wrapped in a valid ftyp+moov+mdat MP4 container.
"""
import struct
import io
import os

def write_box(f, box_type: bytes, data: bytes):
    """Write an MP4 box (atom) with length header."""
    size = 8 + len(data)
    f.write(struct.pack('>I', size))
    f.write(box_type)
    f.write(data)

def make_box(box_type: bytes, data: bytes) -> bytes:
    """Create an MP4 box and return as bytes."""
    buf = io.BytesIO()
    write_box(buf, box_type, data)
    return buf.getvalue()

def make_full_box(box_type: bytes, version: int, flags: int, data: bytes) -> bytes:
    """Create a full box with version and flags."""
    header = struct.pack('>I', (version << 24) | flags)
    return make_box(box_type, header + data)

def encode_h264_sps_pps(width, height, y_val, u_val, v_val):
    """
    Create a minimal H.264 Baseline SPS, PPS, and a single IDR slice
    that produces a solid color frame.
    
    This uses Exp-Golomb coding for the SPS/PPS parameters and
    creates a single I-frame IDR slice with all macroblocks set to
    the same color using I_PCM mode.
    """
    # For simplicity, we'll create a minimal valid H.264 bitstream
    # with SPS + PPS + IDR slice
    
    mb_width = (width + 15) // 16
    mb_height = (height + 15) // 16
    actual_width = mb_width * 16
    actual_height = mb_height * 16
    
    # --- SPS (Sequence Parameter Set) ---
    # NAL unit type 7
    sps = bytearray()
    
    class BitWriter:
        def __init__(self):
            self.data = bytearray()
            self.current_byte = 0
            self.bit_pos = 7
            
        def write_bit(self, b):
            if b:
                self.current_byte |= (1 << self.bit_pos)
            self.bit_pos -= 1
            if self.bit_pos < 0:
                self.data.append(self.current_byte)
                self.current_byte = 0
                self.bit_pos = 7
                
        def write_bits(self, val, n):
            for i in range(n - 1, -1, -1):
                self.write_bit((val >> i) & 1)
                
        def write_ue(self, val):
            """Write unsigned Exp-Golomb coded value."""
            val += 1
            n = val.bit_length()
            for _ in range(n - 1):
                self.write_bit(0)
            self.write_bits(val, n)
            
        def write_se(self, val):
            """Write signed Exp-Golomb coded value."""
            if val > 0:
                self.write_ue(2 * val - 1)
            else:
                self.write_ue(-2 * val)
                
        def finalize(self):
            if self.bit_pos != 7:
                # RBSP trailing bits: 1 followed by 0s to byte-align
                self.write_bit(1)
                while self.bit_pos != 7:
                    self.write_bit(0)
            return bytes(self.data)
    
    # Build SPS
    bw = BitWriter()
    # forbidden_zero_bit
    bw.write_bit(0)
    # nal_ref_idc = 3 (high priority)
    bw.write_bits(3, 2)
    # nal_unit_type = 7 (SPS)
    bw.write_bits(7, 5)
    # profile_idc = 66 (Baseline)
    bw.write_bits(66, 8)
    # constraint_set0_flag through constraint_set5_flag + reserved_zero_2bits
    bw.write_bits(0b11000000, 8)
    # level_idc = 10
    bw.write_bits(10, 8)
    # seq_parameter_set_id = 0
    bw.write_ue(0)
    # log2_max_frame_num_minus4 = 0
    bw.write_ue(0)
    # pic_order_cnt_type = 0
    bw.write_ue(0)
    # log2_max_pic_order_cnt_lsb_minus4 = 0
    bw.write_ue(0)
    # max_num_ref_frames = 0
    bw.write_ue(0)
    # gaps_in_frame_num_value_allowed_flag = 0
    bw.write_bit(0)
    # pic_width_in_mbs_minus1
    bw.write_ue(mb_width - 1)
    # pic_height_in_map_units_minus1
    bw.write_ue(mb_height - 1)
    # frame_mbs_only_flag = 1 (progressive)
    bw.write_bit(1)
    # direct_8x8_inference_flag (not present for baseline, skip)
    # frame_cropping_flag
    if actual_width != width or actual_height != height:
        bw.write_bit(1)
        # crop left
        bw.write_ue(0)
        # crop right
        bw.write_ue((actual_width - width) // 2)
        # crop top
        bw.write_ue(0)
        # crop bottom
        bw.write_ue((actual_height - height) // 2)
    else:
        bw.write_bit(0)
    # vui_parameters_present_flag = 0
    bw.write_bit(0)
    sps_data = bw.finalize()
    
    # Build PPS
    bw2 = BitWriter()
    # forbidden_zero_bit
    bw2.write_bit(0)
    # nal_ref_idc = 3
    bw2.write_bits(3, 2)
    # nal_unit_type = 8 (PPS)
    bw2.write_bits(8, 5)
    # pic_parameter_set_id = 0
    bw2.write_ue(0)
    # seq_parameter_set_id = 0
    bw2.write_ue(0)
    # entropy_coding_mode_flag = 0 (CAVLC for baseline)
    bw2.write_bit(0)
    # bottom_field_pic_order_in_frame_present_flag = 0
    bw2.write_bit(0)
    # num_slice_groups_minus1 = 0
    bw2.write_ue(0)
    # num_ref_idx_l0_default_active_minus1 = 0
    bw2.write_ue(0)
    # num_ref_idx_l1_default_active_minus1 = 0
    bw2.write_ue(0)
    # weighted_pred_flag = 0
    bw2.write_bit(0)
    # weighted_bipred_idc = 0
    bw2.write_bits(0, 2)
    # pic_init_qp_minus26 = 0
    bw2.write_se(0)
    # pic_init_qs_minus26 = 0
    bw2.write_se(0)
    # chroma_qp_index_offset = 0
    bw2.write_se(0)
    # deblocking_filter_control_present_flag = 0
    bw2.write_bit(0)
    # constrained_intra_pred_flag = 0
    bw2.write_bit(0)
    # redundant_pic_cnt_present_flag = 0
    bw2.write_bit(0)
    pps_data = bw2.finalize()
    
    # Build IDR slice with I_PCM macroblocks
    bw3 = BitWriter()
    # forbidden_zero_bit
    bw3.write_bit(0)
    # nal_ref_idc = 3
    bw3.write_bits(3, 2)
    # nal_unit_type = 5 (IDR slice)
    bw3.write_bits(5, 5)
    # first_mb_in_slice = 0
    bw3.write_ue(0)
    # slice_type = 7 (I slice, all slices are I)
    bw3.write_ue(7)
    # pic_parameter_set_id = 0
    bw3.write_ue(0)
    # frame_num = 0
    bw3.write_bits(0, 4)  # log2_max_frame_num = 4
    # idr_pic_id = 0
    bw3.write_ue(0)
    # pic_order_cnt_lsb = 0
    bw3.write_bits(0, 4)  # log2_max_pic_order_cnt_lsb = 4
    # dec_ref_pic_marking: no_output_of_prior_pics_flag=0, long_term_reference_flag=0
    bw3.write_bit(0)
    bw3.write_bit(0)
    # slice_qp_delta = 0
    bw3.write_se(0)
    
    # Now encode macroblocks using I_PCM (mb_type = 25 for I slices)
    total_mbs = mb_width * mb_height
    for mb_idx in range(total_mbs):
        if mb_idx > 0:
            # mb_skip_run is not used in I slices; for subsequent MBs we just continue
            pass
        # mb_type = 25 (I_PCM) - ue(25)
        bw3.write_ue(25)
        
        # Byte-align before PCM data
        if bw3.bit_pos != 7:
            while bw3.bit_pos != 7:
                bw3.write_bit(0)
        
        # Write 256 luma samples (16x16)
        for _ in range(256):
            bw3.write_bits(y_val, 8)
        # Write 64 Cb samples (8x8)
        for _ in range(64):
            bw3.write_bits(u_val, 8)
        # Write 64 Cr samples (8x8)
        for _ in range(64):
            bw3.write_bits(v_val, 8)
    
    # RBSP trailing bits
    slice_data = bw3.finalize()
    
    # Emulation prevention: escape 00 00 00, 00 00 01, 00 00 02, 00 00 03
    def escape_rbsp(data):
        out = bytearray()
        i = 0
        while i < len(data):
            if i + 2 < len(data) and data[i] == 0 and data[i+1] == 0 and data[i+2] in (0, 1, 2, 3):
                out.append(0)
                out.append(0)
                out.append(3)  # emulation_prevention_three_byte
                out.append(data[i+2])
                i += 3
            else:
                out.append(data[i])
                i += 1
        return bytes(out)
    
    sps_escaped = escape_rbsp(sps_data)
    pps_escaped = escape_rbsp(pps_data)
    slice_escaped = escape_rbsp(slice_data)
    
    return sps_escaped, pps_escaped, slice_escaped, actual_width, actual_height


def rgb_to_yuv(r, g, b):
    """Convert RGB to YUV (BT.601)."""
    y = int(0.299 * r + 0.587 * g + 0.114 * b)
    u = int(-0.169 * r - 0.331 * g + 0.500 * b + 128)
    v = int(0.500 * r - 0.419 * g - 0.081 * b + 128)
    return max(0, min(255, y)), max(0, min(255, u)), max(0, min(255, v))


def create_mp4(filename, width, height, r, g, b, duration_ms=2000, fps=1):
    """Create a minimal valid MP4 file with a solid color frame."""
    y, u, v = rgb_to_yuv(r, g, b)
    sps_data, pps_data, slice_data, actual_w, actual_h = encode_h264_sps_pps(
        width, height, y, u, v
    )
    
    # Strip the NAL header byte from SPS/PPS for avcC (it's included in the NAL data)
    # SPS data includes the NAL header; for avcC we need it WITH the NAL type byte
    sps_nalu = sps_data  # includes NAL header byte
    pps_nalu = pps_data
    
    # Build complete H.264 stream with start codes for mdat
    # Use length-prefixed NALUs (4-byte length prefix) for MP4
    mdat_content = bytearray()
    
    # IDR slice NALU
    nalu_length = len(slice_data)
    mdat_content += struct.pack('>I', nalu_length)
    mdat_content += slice_data
    
    mdat_data = bytes(mdat_content)
    
    timescale = 1000
    sample_duration = duration_ms  # one frame for the full duration
    
    # Build the MP4 structure
    f = io.BytesIO()
    
    # ftyp box
    ftyp_data = b'isom'  # major_brand
    ftyp_data += struct.pack('>I', 0x200)  # minor_version
    ftyp_data += b'isomiso2avc1mp41'  # compatible_brands
    write_box(f, b'ftyp', ftyp_data)
    
    # Build moov box
    # mvhd (Movie Header)
    mvhd = struct.pack('>I', 0)  # version=0, flags=0
    mvhd += struct.pack('>I', 0)  # creation_time
    mvhd += struct.pack('>I', 0)  # modification_time
    mvhd += struct.pack('>I', timescale)  # timescale
    mvhd += struct.pack('>I', duration_ms)  # duration
    mvhd += struct.pack('>I', 0x00010000)  # rate (1.0)
    mvhd += struct.pack('>H', 0x0100)  # volume (1.0)
    mvhd += b'\x00' * 10  # reserved
    # Matrix (identity)
    mvhd += struct.pack('>9I',
        0x00010000, 0, 0,
        0, 0x00010000, 0,
        0, 0, 0x40000000)
    mvhd += b'\x00' * 24  # pre_defined
    mvhd += struct.pack('>I', 2)  # next_track_ID
    mvhd_box = make_box(b'mvhd', mvhd)
    
    # Build trak box
    # tkhd (Track Header)
    tkhd = struct.pack('>I', 0x00000003)  # version=0, flags=track_enabled|track_in_movie
    tkhd += struct.pack('>I', 0)  # creation_time
    tkhd += struct.pack('>I', 0)  # modification_time
    tkhd += struct.pack('>I', 1)  # track_ID
    tkhd += struct.pack('>I', 0)  # reserved
    tkhd += struct.pack('>I', duration_ms)  # duration
    tkhd += b'\x00' * 8  # reserved
    tkhd += struct.pack('>H', 0)  # layer
    tkhd += struct.pack('>H', 0)  # alternate_group
    tkhd += struct.pack('>H', 0)  # volume (0 for video)
    tkhd += struct.pack('>H', 0)  # reserved
    # Matrix (identity)
    tkhd += struct.pack('>9I',
        0x00010000, 0, 0,
        0, 0x00010000, 0,
        0, 0, 0x40000000)
    tkhd += struct.pack('>I', width << 16)  # width (fixed point 16.16)
    tkhd += struct.pack('>I', height << 16)  # height (fixed point 16.16)
    tkhd_box = make_box(b'tkhd', tkhd)
    
    # Build mdia box
    # mdhd
    mdhd = struct.pack('>I', 0)  # version=0, flags=0
    mdhd += struct.pack('>I', 0)  # creation_time
    mdhd += struct.pack('>I', 0)  # modification_time
    mdhd += struct.pack('>I', timescale)  # timescale
    mdhd += struct.pack('>I', duration_ms)  # duration
    mdhd += struct.pack('>H', 0x55C4)  # language (undetermined)
    mdhd += struct.pack('>H', 0)  # pre_defined
    mdhd_box = make_box(b'mdhd', mdhd)
    
    # hdlr
    hdlr = struct.pack('>I', 0)  # version=0, flags=0
    hdlr += struct.pack('>I', 0)  # pre_defined
    hdlr += b'vide'  # handler_type
    hdlr += b'\x00' * 12  # reserved
    hdlr += b'VideoHandler\x00'  # name
    hdlr_box = make_box(b'hdlr', hdlr)
    
    # Build minf box
    # vmhd
    vmhd = struct.pack('>I', 0x00000001)  # version=0, flags=1
    vmhd += struct.pack('>H', 0)  # graphicsmode
    vmhd += struct.pack('>3H', 0, 0, 0)  # opcolor
    vmhd_box = make_box(b'vmhd', vmhd)
    
    # dinf > dref
    dref_entry = make_box(b'url ', struct.pack('>I', 0x00000001))  # self-contained
    dref = struct.pack('>I', 0)  # version=0, flags=0
    dref += struct.pack('>I', 1)  # entry_count
    dref += dref_entry
    dref_box = make_box(b'dref', dref)
    dinf_box = make_box(b'dinf', dref_box)
    
    # stbl (Sample Table)
    # stsd (Sample Description)
    # avcC (AVC Decoder Configuration Record)
    avcc = bytearray()
    avcc.append(1)  # configurationVersion
    avcc.append(sps_nalu[1] if len(sps_nalu) > 1 else 66)  # AVCProfileIndication
    avcc.append(sps_nalu[2] if len(sps_nalu) > 2 else 0)  # profile_compatibility
    avcc.append(sps_nalu[3] if len(sps_nalu) > 3 else 10)  # AVCLevelIndication
    avcc.append(0xFF)  # lengthSizeMinusOne = 3 (4 bytes) | reserved 0b111111
    avcc.append(0xE1)  # numOfSequenceParameterSets = 1 | reserved 0b111
    avcc += struct.pack('>H', len(sps_nalu))  # SPS length
    avcc += sps_nalu
    avcc.append(1)  # numOfPictureParameterSets
    avcc += struct.pack('>H', len(pps_nalu))  # PPS length
    avcc += pps_nalu
    avcc_box = make_box(b'avcC', bytes(avcc))
    
    # avc1 (Visual Sample Entry)
    avc1 = bytearray()
    avc1 += b'\x00' * 6  # reserved
    avc1 += struct.pack('>H', 1)  # data_reference_index
    avc1 += struct.pack('>H', 0)  # pre_defined
    avc1 += struct.pack('>H', 0)  # reserved
    avc1 += b'\x00' * 12  # pre_defined
    avc1 += struct.pack('>H', width)  # width
    avc1 += struct.pack('>H', height)  # height
    avc1 += struct.pack('>I', 0x00480000)  # horizresolution (72 dpi)
    avc1 += struct.pack('>I', 0x00480000)  # vertresolution (72 dpi)
    avc1 += struct.pack('>I', 0)  # reserved
    avc1 += struct.pack('>H', 1)  # frame_count
    avc1 += b'\x00' * 32  # compressorname
    avc1 += struct.pack('>H', 0x0018)  # depth
    avc1 += struct.pack('>h', -1)  # pre_defined
    avc1 += avcc_box
    avc1_box = make_box(b'avc1', bytes(avc1))
    
    stsd = struct.pack('>I', 0)  # version=0, flags=0
    stsd += struct.pack('>I', 1)  # entry_count
    stsd += avc1_box
    stsd_box = make_box(b'stsd', stsd)
    
    # stts (Time-to-Sample)
    stts = struct.pack('>I', 0)  # version=0, flags=0
    stts += struct.pack('>I', 1)  # entry_count
    stts += struct.pack('>I', 1)  # sample_count
    stts += struct.pack('>I', sample_duration)  # sample_delta
    stts_box = make_box(b'stts', stts)
    
    # stss (Sync Sample) - mark our single sample as a sync sample
    stss = struct.pack('>I', 0)  # version=0, flags=0
    stss += struct.pack('>I', 1)  # entry_count
    stss += struct.pack('>I', 1)  # sample_number
    stss_box = make_box(b'stss', stss)
    
    # stsc (Sample-to-Chunk)
    stsc = struct.pack('>I', 0)  # version=0, flags=0
    stsc += struct.pack('>I', 1)  # entry_count
    stsc += struct.pack('>I', 1)  # first_chunk
    stsc += struct.pack('>I', 1)  # samples_per_chunk
    stsc += struct.pack('>I', 1)  # sample_description_index
    stsc_box = make_box(b'stsc', stsc)
    
    # stsz (Sample Size)
    stsz = struct.pack('>I', 0)  # version=0, flags=0
    stsz += struct.pack('>I', 0)  # sample_size (0 = variable)
    stsz += struct.pack('>I', 1)  # sample_count
    stsz += struct.pack('>I', len(mdat_data))  # entry_size
    stsz_box = make_box(b'stsz', stsz)
    
    # Calculate mdat offset (we need to know the moov size first)
    # We'll use a placeholder and fix it later
    # stco (Chunk Offset) - placeholder
    stco = struct.pack('>I', 0)  # version=0, flags=0
    stco += struct.pack('>I', 1)  # entry_count
    stco += struct.pack('>I', 0)  # chunk_offset (placeholder)
    stco_box = make_box(b'stco', stco)
    
    # Assemble stbl
    stbl_box = make_box(b'stbl', stsd_box + stts_box + stss_box + stsc_box + stsz_box + stco_box)
    
    # Assemble minf
    minf_box = make_box(b'minf', vmhd_box + dinf_box + stbl_box)
    
    # Assemble mdia
    mdia_box = make_box(b'mdia', mdhd_box + hdlr_box + minf_box)
    
    # Assemble trak
    trak_box = make_box(b'trak', tkhd_box + mdia_box)
    
    # Assemble moov
    moov_box = make_box(b'moov', mvhd_box + trak_box)
    
    # Now calculate the mdat offset
    # ftyp box size + moov box size + 8 (mdat header)
    ftyp_box = f.getvalue()  # already written
    mdat_offset = len(ftyp_box) + len(moov_box) + 8  # +8 for mdat box header
    
    # Fix the stco chunk offset
    # Find stco in moov_box and patch it
    moov_bytes = bytearray(moov_box)
    # Search for 'stco' in the moov box
    stco_pos = moov_bytes.find(b'stco')
    if stco_pos >= 0:
        # The offset is at stco_pos + 4 (box_type) + 4 (version/flags) + 4 (entry_count)
        offset_pos = stco_pos + 4 + 4 + 4
        struct.pack_into('>I', moov_bytes, offset_pos, mdat_offset)
    moov_box = bytes(moov_bytes)
    
    # Write the final file
    with open(filename, 'wb') as out:
        out.write(ftyp_box)
        out.write(moov_box)
        # mdat box
        mdat_size = 8 + len(mdat_data)
        out.write(struct.pack('>I', mdat_size))
        out.write(b'mdat')
        out.write(mdat_data)
    
    file_size = len(ftyp_box) + len(moov_box) + 8 + len(mdat_data)
    return file_size


def main():
    output_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Define 8 videos with different colors  
    videos = [
        ("video1.mp4", 160, 120, 255, 0, 0, "Red"),
        ("video2.mp4", 160, 120, 0, 255, 0, "Green"),
        ("video3.mp4", 160, 120, 0, 0, 255, "Blue"),
        ("video4.mp4", 160, 120, 255, 255, 0, "Yellow"),
        ("video5.mp4", 160, 120, 255, 0, 255, "Magenta"),
        ("video6.mp4", 160, 120, 0, 255, 255, "Cyan"),
        ("video7.mp4", 160, 120, 128, 128, 128, "Gray"),
        ("video8.mp4", 160, 120, 255, 128, 0, "Orange"),
    ]
    
    for name, w, h, r, g, b, color in videos:
        filepath = os.path.join(output_dir, name)
        size = create_mp4(filepath, w, h, r, g, b, duration_ms=3000)
        print(f"Created {name}: {color} {w}x{h}, {size} bytes ({size/1024:.1f} KB)")
    
    print(f"\nAll videos created in: {output_dir}")


if __name__ == '__main__':
    main()
