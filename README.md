```

the claude incident of 3.4.3 made me want to see how the rsync algo works and
if i could reimplement it. primary focus is the algorithm but getting it to
work over networks would be nice

see https://en.wikipedia.org/wiki/Rsync >>>

By default, rsync determines which files differ between the sending and
receiving systems by checking the modification time and size of each file. If
time or size is different between the systems, it transfers the file from the
sending to the receiving system. As this only requires reading file directory
information, it is quick, but it will miss unusual modifications which change
neither.[7]

<<<

General algorithm
see https://raw.githubusercontent.com/RsyncProject/rsync/refs/heads/v3.4-stable/tech_report.tex >>>

1. β splits the file B into a series of non-overlapping fixed-sized blocks of
   size S bytes The last block may be shorter than S bytes.

2. For each of these blocks β calculates two checksums: a weak “rolling” 32-bit
   checksum (described below) and a strong 128-bit MD4 checksum.

3. β sends these checksums to α.

4. α searches through A to find all blocks of length S bytes (at any offset,
   not just multiples of S) that have the same weak and strong checksum as one
   of the blocks of B. This can be done in a single pass very quickly using a
   special property of the rolling checksum described below.

5. α sends β a sequence of instructions for constructing a copy of A. Each
   instruction is either a reference to a block of B, or literal data. Literal
   data is sent only for those sections of A which did not match any of the
   blocks of B.

<<<

simpler terms:
* calc checksums of DST
* send those checksums to SRC
* calc ROLLING checksum for the blocks of SRC if they match, then calculate longer checksum to ensure
* send data + references to DST to be reconstructed

```


