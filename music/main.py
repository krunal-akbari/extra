from pytube import YouTube
import sys
l = sys.argv

class Music:

    """
        this is use for givin music download and showing the personal list
        i hope i can use this with any os
    """

    def __init__(self) -> None:
        self.types=  dict()

    def DictMaker(self):
        with open("types.txt") as file:
            for x in file:
                a = x.strip().split()
                with open(a[1]) as f:
                    songs = dict()
                    for x in f.readlines():
                        x  = x.split('|')
                        songs[x[0].strip()]= x[1].strip()
                self.types[a[0]] = songs

        
    def Url(self):
        if  not self.types :
            self.DictMaker()
        for _ in self.types:
            for x in self.types[_]:
                self.SongDownloader(self.types[_][x],_)
 
    def SongDownloader(self,url,folder):
        yt = YouTube(url)
        print(yt.title)
        yt.streams.filter(only_audio=True).first().download("songs/" + folder)

if len(l) >=2 :
    if "download" in l:
        test = Music()  
        test.Url()

