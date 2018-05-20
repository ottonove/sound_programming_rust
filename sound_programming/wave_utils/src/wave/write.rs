extern crate byteorder;
use wave::Pcm;
use wave::WaveData;
use self::byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;

fn write_i8x4<T>(mut fp: T, arr: [i8; 4])
where
    T: byteorder::WriteBytesExt,
{
    for item in arr.iter() {
        fp.write_i8(*item).unwrap();
    }
}

#[allow(non_snake_case)]
pub fn wave_write_header<T, U>(path: &str, pcm: &T) -> File
where
    T: Pcm,
    U: WaveData,
{
    let channel_: i32 = T::CHANNEL;

    let riff_chunk_ID: [i8; 4] = ['R' as i8, 'I' as i8, 'F' as i8, 'F' as i8];
    let riff_chunk_size: i32 = U::MYSTERIOUS + pcm.get_length() as i32 * U::BYTE_NUM * channel_;
    let file_format_type: [i8; 4] = ['W' as i8, 'A' as i8, 'V' as i8, 'E' as i8];
    let fmt_chunk_ID: [i8; 4] = ['f' as i8, 'm' as i8, 't' as i8, ' ' as i8];
    let fmt_chunk_size: i32 = U::CHUNK_SIZE;
    let wave_format_type: i16 = U::WAVE_FORMAT_TYPE;
    let channel: i16 = channel_ as i16;
    let samples_per_sec: i32 = pcm.get_fs() as i32; /* 標本化周波数 */
    let bytes_per_sec: i32 = pcm.get_fs() as i32 * pcm.get_bits() / 8 * channel_;
    let block_size: i16 = (pcm.get_bits() / 8) as i16 * channel;
    let bits_per_sample: i16 = pcm.get_bits() as i16; /* 量子化精度 */
    let data_chunk_ID: [i8; 4] = ['d' as i8, 'a' as i8, 't' as i8, 'a' as i8];
    let data_chunk_size: i32 = pcm.get_length() as i32 * U::BYTE_NUM * channel_;

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
    if U::CHUNK_SIZE > 16 {
        let extra_size: i16 = 0;
        let fact_chunk_ID: [i8; 4] = ['f' as i8, 'a' as i8, 'c' as i8, 't' as i8];
        let fact_chunk_size: i32 = 4;
        let sample_length: i32 = pcm.get_length() as i32;

        fp.write_i16::<LittleEndian>(extra_size).unwrap();
        write_i8x4(&mut fp, fact_chunk_ID);
        fp.write_i32::<LittleEndian>(fact_chunk_size).unwrap();
        fp.write_i32::<LittleEndian>(sample_length).unwrap();
    }

    write_i8x4(&mut fp, data_chunk_ID);
    fp.write_i32::<LittleEndian>(data_chunk_size).unwrap();
    return fp;
}

