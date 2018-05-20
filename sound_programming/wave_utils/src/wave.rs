extern crate byteorder;
use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use MONO_PCM;
use MONO_PCM_CONST;
use MonoPcm;
use STEREO_PCM_CONST;
use StereoPcm;
use libc::c_char;
use std::fs::File;
use std::mem;
use std::slice::from_raw_parts_mut;
use to_c_str;

fn read_i8x4<T>(mut fp: T) -> [i8; 4]
where
    T: byteorder::ReadBytesExt,
{
    let mut arr = [0; 4];
    for item in arr.iter_mut() {
        *item = fp.read_i8().unwrap();
    }
    return arr;
}

fn write_i8x4<T>(mut fp: T, arr: [i8; 4])
where
    T: byteorder::WriteBytesExt,
{
    for item in arr.iter() {
        fp.write_i8(*item).unwrap();
    }
}

#[allow(non_snake_case)]
fn read_header(file_name: &str) -> (File, i32, i16, i32) {
    let mut fp = File::open(file_name).expect("file not found");

    let _riff_chunk_ID = read_i8x4(&mut fp);
    let _riff_chunk_size = fp.read_i32::<LittleEndian>().unwrap();
    let _file_format_type = read_i8x4(&mut fp);
    let _fmt_chunk_ID = read_i8x4(&mut fp);
    let _fmt_chunk_size = fp.read_i32::<LittleEndian>().unwrap();
    let _wave_format_type = fp.read_i16::<LittleEndian>().unwrap();
    let _channel = fp.read_i16::<LittleEndian>().unwrap();
    let samples_per_sec = fp.read_i32::<LittleEndian>().unwrap();
    let _bytes_per_sec = fp.read_i32::<LittleEndian>().unwrap();
    let _block_size = fp.read_i16::<LittleEndian>().unwrap();
    let bits_per_sample = fp.read_i16::<LittleEndian>().unwrap();
    let _data_chunk_ID = read_i8x4(&mut fp);
    let data_chunk_size = fp.read_i32::<LittleEndian>().unwrap();

    return (fp, samples_per_sec, bits_per_sample, data_chunk_size);
}

//not tested
pub fn wave_read_8bit_mono_safer3(path: &str) -> MonoPcm {
    let (mut fp, pcm_fs, pcm_bits, data_chunk_size) = read_header(path);
    let pcm_length = data_chunk_size as usize;
    let mut pcm_s = vec![0.0; pcm_length];

    for n in 0..pcm_length {
        let data = fp.read_u8().unwrap();
        pcm_s[n] = (data as f64 - 128.0) / 128.0; /* 音データを-1以上1未満の範囲に正規化する */
    }

    return MonoPcm {
        s: pcm_s,
        fs: pcm_fs as usize,
        bits: pcm_bits as i32,
        length: pcm_length as usize,
    };
}

// not tested
#[allow(non_snake_case)]
pub fn wave_read_8bit_stereo_safer3(path: &str) -> StereoPcm {
    let (mut fp, pcm_fs, pcm_bits, data_chunk_size) = read_header(path);
    let pcm_length = (data_chunk_size / 2) as usize; /* 音データの長さ */

    let mut pcm_sL = vec![0.0; pcm_length];
    let mut pcm_sR = vec![0.0; pcm_length];
    for n in 0..pcm_length {
        let data = fp.read_u8().unwrap();
        pcm_sL[n] = (data as f64 - 128.0) / 128.0; /* 音データを-1以上1未満の範囲に正規化する */
        let data = fp.read_u8().unwrap();
        pcm_sR[n] = (data as f64 - 128.0) / 128.0; /* 音データを-1以上1未満の範囲に正規化する */
    }
    return StereoPcm {
        s_l: pcm_sL,
        s_r: pcm_sR,
        fs: pcm_fs as usize,
        bits: pcm_bits as i32,
        length: pcm_length as usize,
    };
}

pub fn wave_read_16bit_mono_safer3(path: &str) -> MonoPcm {
    let (mut fp, pcm_fs, pcm_bits, data_chunk_size) = read_header(path);
    let pcm_length = (data_chunk_size / 2) as usize;
    let mut pcm_s = vec![0.0; pcm_length];

    for n in 0..pcm_length {
        let data = fp.read_i16::<LittleEndian>().unwrap();
        pcm_s[n] = (data as f64) / 32768.0; /* 音データを-1以上1未満の範囲に正規化する */
    }

    return MonoPcm {
        s: pcm_s,
        fs: pcm_fs as usize,
        bits: pcm_bits as i32,
        length: pcm_length as usize,
    };
}

#[allow(non_snake_case)]
pub fn wave_read_16bit_stereo_safer3(path: &str) -> StereoPcm {
    let (mut fp, pcm_fs, pcm_bits, data_chunk_size) = read_header(path);
    let pcm_length = (data_chunk_size / 4) as usize; /* 音データの長さ */

    let mut pcm_sL = vec![0.0; pcm_length];
    let mut pcm_sR = vec![0.0; pcm_length];
    for n in 0..pcm_length {
        let data = fp.read_i16::<LittleEndian>().unwrap();
        pcm_sL[n] = data as f64 / 32768.0; /* 音データを-1以上1未満の範囲に正規化する */
        let data = fp.read_i16::<LittleEndian>().unwrap();
        pcm_sR[n] = data as f64 / 32768.0; /* 音データを-1以上1未満の範囲に正規化する */
    }

    return StereoPcm {
        s_l: pcm_sL,
        s_r: pcm_sR,
        fs: pcm_fs as usize,
        bits: pcm_bits as i32,
        length: pcm_length as usize,
    };
}

#[link(name = "wave")]
extern "C" {
    pub fn wave_write_8bit_mono(pcm: *const MONO_PCM_CONST, file_name: *const c_char);
    pub fn wave_write_8bit_stereo(pcm: *const STEREO_PCM_CONST, file_name: *const c_char);

    /*fn wave_write_16bit_stereo(pcm: *const STEREO_PCM_CONST, file_name: *const c_char);
    */
    fn wave_read_PCMA_mono(pcm: *mut MONO_PCM, file_name: *const c_char);
    fn wave_write_PCMA_mono(pcm: *const MONO_PCM_CONST, file_name: *const c_char);
    fn wave_read_IMA_ADPCM_mono(pcm: *mut MONO_PCM, file_name: *const c_char);
    fn wave_write_IMA_ADPCM_mono(pcm: *const MONO_PCM_CONST, file_name: *const c_char);
    fn wave_read_PCMU_mono(pcm: *mut MONO_PCM, file_name: *const c_char);
    fn wave_write_PCMU_mono(pcm: *const MONO_PCM_CONST, file_name: *const c_char);

}

trait WaveData {
    fn convert_from_float(d: f64) -> Self;
}

impl WaveData for i16 {
    fn convert_from_float(d: f64) -> i16 {
        let mut s = (d + 1.0) / 2.0 * 65536.0;

        if s > 65535.0 {
            s = 65535.0; /* クリッピング */
        } else if s < 0.0 {
            s = 0.0; /* クリッピング */
        }

        ((s + 0.5) as i32 - 32768) as i16 /* 四捨五入とオフセットの調節 */
    }
}

#[allow(non_snake_case)]
pub fn wave_read_IMA_ADPCM_mono_safer3(path: &str) -> MonoPcm {
    unsafe {
        let mut pcm: MONO_PCM = mem::uninitialized();
        wave_read_IMA_ADPCM_mono(&mut pcm, to_c_str(path));
        MonoPcm {
            fs: pcm.fs as usize,
            bits: pcm.bits,
            length: pcm.length as usize,
            s: from_raw_parts_mut(pcm.s, pcm.length as usize).to_vec(),
        }
    }
}

#[allow(non_snake_case)]
pub fn wave_write_IMA_ADPCM_mono_safer3(path: &str, pcm: &MonoPcm) {
    let pcm1: MONO_PCM_CONST = MONO_PCM_CONST {
        fs: pcm.fs as i32,
        bits: pcm.bits,
        length: pcm.length as i32,
        s: pcm.s.as_ptr(),
    };
    unsafe {
        wave_write_IMA_ADPCM_mono(&pcm1, to_c_str(path));
    }
}

#[allow(non_snake_case)]
pub fn wave_write_8bit_mono_safer3(path: &str, pcm: &MonoPcm) {
    let pcm1: MONO_PCM_CONST = MONO_PCM_CONST {
        fs: pcm.fs as i32,
        bits: pcm.bits,
        length: pcm.length as i32,
        s: pcm.s.as_ptr(),
    };
    unsafe {
        wave_write_8bit_mono(&pcm1, to_c_str(path));
    }
}


#[allow(non_snake_case)]
pub fn wave_read_PCMU_mono_safer3(path: &str) -> MonoPcm {
    unsafe {
        let mut pcm: MONO_PCM = mem::uninitialized();
        wave_read_PCMU_mono(&mut pcm, to_c_str(path));
        MonoPcm {
            fs: pcm.fs as usize,
            bits: pcm.bits,
            length: pcm.length as usize,
            s: from_raw_parts_mut(pcm.s, pcm.length as usize).to_vec(),
        }
    }
}

#[allow(non_snake_case)]
pub fn wave_write_PCMU_mono_safer3(path: &str, pcm: &MonoPcm) {
    let pcm1: MONO_PCM_CONST = MONO_PCM_CONST {
        fs: pcm.fs as i32,
        bits: pcm.bits,
        length: pcm.length as i32,
        s: pcm.s.as_ptr(),
    };
    unsafe {
        wave_write_PCMU_mono(&pcm1, to_c_str(path));
    }
}

#[allow(non_snake_case)]
pub fn wave_read_PCMA_mono_safer3(path: &str) -> MonoPcm {
    unsafe {
        let mut pcm: MONO_PCM = mem::uninitialized();
        wave_read_PCMA_mono(&mut pcm, to_c_str(path));
        MonoPcm {
            fs: pcm.fs as usize,
            bits: pcm.bits,
            length: pcm.length as usize,
            s: from_raw_parts_mut(pcm.s, pcm.length as usize).to_vec(),
        }
    }
}

#[allow(non_snake_case)]
pub fn wave_write_PCMA_mono_safer3(path: &str, pcm: &MonoPcm) {
    let pcm1: MONO_PCM_CONST = MONO_PCM_CONST {
        fs: pcm.fs as i32,
        bits: pcm.bits,
        length: pcm.length as i32,
        s: pcm.s.as_ptr(),
    };
    unsafe {
        wave_write_PCMA_mono(&pcm1, to_c_str(path));
    }
}

#[allow(non_snake_case)]
pub fn wave_write_16bit_stereo_safer3(path: &str, pcm: &StereoPcm) {
    let mut fp = wave_write_16bit_header(path, pcm);
    for n in 0..pcm.length {
        fp.write_i16::<LittleEndian>(WaveData::convert_from_float(pcm.s_l[n]))
            .unwrap(); /* 音データ（Lチャンネル）の書き出し */
        fp.write_i16::<LittleEndian>(WaveData::convert_from_float(pcm.s_r[n]))
            .unwrap(); /* 音データ（Rチャンネル）の書き出し */
    }
}

pub fn wave_write_16bit_mono_safer3(path: &str, pcm: &MonoPcm) {
    let mut fp = wave_write_16bit_header(path, pcm);
    for n in 0..pcm.get_length() {
        fp.write_i16::<LittleEndian>(WaveData::convert_from_float(pcm.s[n]))
            .unwrap(); /* 音データの書き出し */
    }
}

trait Pcm {
    fn get_fs(&self) -> usize;
    fn get_bits(&self) -> i32;
    fn get_length(&self) -> usize;
    const CHANNEL: i32;
}

impl Pcm for MonoPcm {
    fn get_fs(&self) -> usize {
        self.fs
    }
    fn get_bits(&self) -> i32 {
        self.bits
    }
    fn get_length(&self) -> usize {
        self.length
    }
    const CHANNEL: i32 = 1;
}

impl Pcm for StereoPcm {
    fn get_fs(&self) -> usize {
        self.fs
    }
    fn get_bits(&self) -> i32 {
        self.bits
    }
    fn get_length(&self) -> usize {
        self.length
    }
    const CHANNEL: i32 = 2;
}

#[allow(non_snake_case)]
fn wave_write_16bit_header<T>(path: &str, pcm: &T) -> File
where
    T: Pcm,
{
    let channel_: i32 = T::CHANNEL;
    let riff_chunk_ID: [i8; 4] = ['R' as i8, 'I' as i8, 'F' as i8, 'F' as i8];
    let riff_chunk_size: i32 = 36 + pcm.get_length() as i32 * 2 * channel_;
    let file_format_type: [i8; 4] = ['W' as i8, 'A' as i8, 'V' as i8, 'E' as i8];
    let fmt_chunk_ID: [i8; 4] = ['f' as i8, 'm' as i8, 't' as i8, ' ' as i8];
    let fmt_chunk_size: i32 = 16;
    let wave_format_type: i16 = 1;
    let channel: i16 = channel_ as i16;
    let samples_per_sec: i32 = pcm.get_fs() as i32; /* 標本化周波数 */
    let bytes_per_sec: i32 = pcm.get_fs() as i32 * pcm.get_bits() / 8 * channel_;
    let block_size: i16 = (pcm.get_bits() / 8) as i16 * channel;
    let bits_per_sample: i16 = pcm.get_bits() as i16; /* 量子化精度 */
    let data_chunk_ID: [i8; 4] = ['d' as i8, 'a' as i8, 't' as i8, 'a' as i8];
    let data_chunk_size: i32 = (pcm.get_length() * 2) as i32 * channel_;

    let mut fp = File::create(path).expect("file cannot be created");
    write_i8x4(&mut fp, riff_chunk_ID);
    fp.write_i32::<LittleEndian>(riff_chunk_size).unwrap();
    write_i8x4(&mut fp, file_format_type);
    write_i8x4(&mut fp, fmt_chunk_ID);
    fp.write_i32::<LittleEndian>(fmt_chunk_size).unwrap();
    fp.write_i16::<LittleEndian>(wave_format_type).unwrap();
    fp.write_i16::<LittleEndian>(channel).unwrap();
    fp.write_i32::<LittleEndian>(samples_per_sec).unwrap();
    fp.write_i32::<LittleEndian>(bytes_per_sec).unwrap();
    fp.write_i16::<LittleEndian>(block_size).unwrap();
    fp.write_i16::<LittleEndian>(bits_per_sample).unwrap();
    write_i8x4(&mut fp, data_chunk_ID);
    fp.write_i32::<LittleEndian>(data_chunk_size).unwrap();
    return fp;
}

/*  
  
  
  
  fclose(fp);
}


*/
