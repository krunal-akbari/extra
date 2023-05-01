import pytube
import os
import sys
import json

l = sys.argv


class Music:

    """
        this is use for givin music download and showing the personal list
        i hope i can use this with any os
    """

    def __init__(self) -> None:
        self.types = json.load(open("list.json", "r"))

    def Insert(self):
        song_type = input("Enter the type of song: ")
        song_name = input("Enter the name of song: ")
        song_url = input("Enter the url of song: ")
        self.types[song_url] = {song_type: song_name}
        json.dump(self.types, open("list.json", "w"))

    def Url(self, bool):
        temp = self.list_song()
        if bool:
            for _ in self.types:
                for x in self.types[_]:
                    if self.types[_][x] in temp:
                        pass
                    else:
                        print(f"'{self.types[_][x]}' is downloding")
                        self.SongDownloader(
                            _, x, self.types[_][x]) if bool else None
                        print("Downloaded")

    @staticmethod
    def SongDownloader(url, path, name):
        yt = pytube.YouTube(url)
        yt.streams.filter(only_audio=True).first().download(
            "songs/" + path, filename=name)

    @staticmethod
    def ClearFolder():
        os.system("rm -rf songs/*")

    def list_song(self):
        songs_name = list()
        for x in os.listdir("songs/"):
            songs_name += os.listdir("/songs/" + x)
        return songs_name

    def Diffrece(self):
        temp = self.list_song()
        for _ in self.types:
            for x in self.types[_]:
                if self.types[_][x] in temp:
                    pass
                else:
                    print(f"'{self.types[_][x]}' is not downloaded")


def main():
    test = Music()
    if len(l) >= 2:

        if "download" in l:
            test.Url(True)
        elif "clear" in l:
            test.ClearFolder()
        elif "list" in l:
            print(test.list_song())
        elif "insert" in l:
            test.Insert()
        elif "diff" in l:
            test.Diffrece()


main()
