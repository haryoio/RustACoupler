# PoPeModem

PoPeModemはマイクとスピーカーを使った無線通信を行うためのソフトウェアです｡

## 使い方

Bandを指定して起動すると､そのBandの周波数帯で通信を行います｡
周波数帯は`band1`, `band2`, `band3`の中から選びます｡

双方向通信をする場合は異なるBandを指定してください｡

```bash
./popemodem band1
```

### ビルド

PoPeModemはNightly Rustでビルドする必要があります｡

```bash
git clone https://github.com/haryoiro/popemodem.git
cd popemodem
cargo build --release
```



## LICENSE

MIT LICENSE
