# RustACoupler

音声による双方向通信を行うソフトウェア。

双方向で音声による待ち受けを行う。

マイクで受け取った信号を常時待ち受け。
ネゴシエーション開始の信号だった場合、その旨を相手に伝える信号を送信。
ネゴシエーション開始パケットには相手モデムのアドレスを含む。

------------
preamble    7octets 56bit
SFD         1octet  8bit
---header---
dst address     1octet  8bit
src address     1octet  8bit
---header---
