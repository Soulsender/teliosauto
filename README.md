### TelIOSAuto
> Automatically sends config data to telnet devices based on configurations set

### Example
**config.ini** (native mobaxterm configuration file)
```
[Bookmarks_1]
S3=#98#1%127.0.0.1%4444%%%2%%%%%0%0%%1080%%%0%-1%0#MobaFont%10%0%0%-1%15%236,236,236%30,30,30%180,180,192%0%-1%0%%xterm%-1%0%_Std_Colors_0_%80%24%0%1%-1%<none>%%0%0%-1%0%#0# #-1
```

**config/S3.txt**
```
enable
conf t
hostname ConfiguredByTelIOSAuto
```

Command example:
`teliosauto --config config.ini --devices config/ --tag Bookmarks_1`

![Example config and path layouts](images/example1.png)
![Connecting to example device on localhost](images/example2.png)