# url-decoder

CLI tool to decode URLs back to readable form.

## Installation

```
cargo install url-decoder
```

or if you like to live on the bleeding edge:

```
git clone https://github.com/ritiek/url-decoder
cd url-decoder
cargo install
```

## Usage

```
urldecode <url> [<url> ..]
```

example:

```
$ urldecode https%3A%2F%2Fr1---sn-gwpa-a0ie.googlevideo.com%2Fvideoplayback%3Fexpire%3D1500766555%26mime%3Dvideo%252Fwebm%26key%3Dyt6%26sparams%3Dclen%252Cdur%252Cei%252Cgir%252Cid%252Cinitcwndbps%252Cip%252Cipbits%252Citag%252Clmt%252Cmime%252Cmm%252Cmn%252Cms%252Cmv%252Cpcm2cms%252Cpl252Cratebypass252Crequiressl%252Csource%252Cexpire%26lmt%3D1476667806824212%26dur%3D0.000%26mt%3D1500744770%26mv%3Dm%26ms%3Dau%26source%3Dyoutube%26signature%3D4DFF0C9FDAB22D3423E8A39DE1F19C65EDAF8EF0.AD688E43F62E832ED6EFEFAFFE24EA4D6FD3B214%26initcwndbps%3D306250%26clen%3D28564761%26gir%3Dyes%26mn%3Dsn-gwpa-a0ie%26id%3Do-AK6vpieeD5Omh-7qNqdhGzesnWQkvKEOvuGl84AgtrZI%26ipbits%3D0%26pcm2cms%3Dyes%26mm%3D31%26ei%3D-oxzWY7IN4mIoAOQqo7IAQ%26pl%3D22%26ratebypass%3Dyes%26itag%3D43

https://r1---sn-gwpa-a0ie.googlevideo.com/videoplayback?expire=1500766555&mime=video%2Fwebm&key=yt6&sparams=clen%2Cdur%2Cei%2Cgir%2Cid%2Cinitcwndbps%2Cip%2Cipbits%2Citag%2Clmt%2Cmime%2Cmm%2Cmn%2Cms%2Cmv%2Cpcm2cms%2Cpl252Cratebypass252Crequiressl%2Csource%2Cexpire&lmt=1476667806824212&dur=0.000&mt=1500744770&mv=m&ms=au&source=youtube&signature=4DFF0C9FDAB22D3423E8A39DE1F19C65EDAF8EF0.AD688E43F62E832ED6EFEFAFFE24EA4D6FD3B214&initcwndbps=306250&clen=28564761&gir=yes&mn=sn-gwpa-a0ie&id=o-AK6vpieeD5Omh-7qNqdhGzesnWQkvKEOvuGl84AgtrZI&ipbits=0&pcm2cms=yes&mm=31&ei=-oxzWY7IN4mIoAOQqo7IAQ&pl=22&ratebypass=yes&itag=43
```

## License

`The MIT License`
