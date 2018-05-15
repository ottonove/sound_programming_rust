# sound_programming_rust
[「サウンドプログラミング入門――音響合成の基本とC言語による実装」](https://www.amazon.co.jp/dp/4774155225)のサンプルプログラムRustへの移植

## 内容
[著者様が配布されているサンプルプログラム](http://floor13.sakura.ne.jp/book06/book06.html)をRustに勝手に移植してみる。特に、ライブラリとしての使用が意図されていると思われる `wave.h` (67回登場。テスト済、未移植), `sinc.h` (12回登場。テスト済、移植済), `iir_filter.h` (10回登場), `adsr.h`(8回登場), `window_function.h` (7回登場。テスト済、移植済), `fft.h` (4回登場。テスト済、移植済), `fir_filter.h` (4回登場) をメインに移植する。
なお、作者様に連絡が取れていないため、何かあったら公開を停止する。
