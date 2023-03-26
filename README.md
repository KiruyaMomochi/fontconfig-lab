# Fontconfig Lab

This lab compares two ways to find **font directories** from fontconfig on a **Linux** system.

1. **FFI**: Use the original [fontconfig](https://fontconfig.org/) C library. \
    This uses <https://github.com/yeslogic/fontconfig-rs>.
2. **Parser**: Parse fonts.conf in pure Rust. \
    This uses <https://github.com/Riey/fontconfig-parser>, which is then used by fontdb.

Things not covered:

- use fontconfig to actually match fonts, honoring blacklist and whitelist, or
- use fontconfig on other platforms like Windows, or even
- rely on the fontconfig cache to recognize fonts on all platforms, like LaTeX does.

## Result

It seems that their results are almost same, although
- the order may differ, and
- `libfontconfig` also considers fonts under `$XDG_DATA_DIRS/share/fonts`. However, none of them exists on my OS.

Here is the result of `cargo run` on my NixOS:

```
Font directories diff of FFI...Parser:

[+] Insert [-] Delete
[=] Both has but order differs
[x] Not exist in filesystem

+ [= ] /nix/store/3cg59xc331qda1f7ganxsc7agfp5xcbm-noto-fonts-20201206-phase3
  [  ] /nix/store/p400pqjqjknhbbhyhsvl6j26qb4hpv0p-noto-fonts-cjk-sans-2.004
+ [= ] /nix/store/afyid3i7s7jq2gk2l0699znlvhz1csk3-noto-fonts-emoji-2.038
+ [= ] /nix/store/fqinqima2iw0nkl9zk36v5dygbg57669-liberation-fonts-2.1.0
  [  ] /nix/store/wq9f5n644wfhrs3cmkbdqrcd5r9bm3v2-fira-code-6.2
  [  ] /nix/store/127g6zfradwp36my8f0xj7lpdkpg15vl-fira-code-symbols-20160811
  [  ] /nix/store/xiiyhpq62bpphbn5syawz1ycxj5yj889-sarasa-gothic-0.40.3
  [  ] /nix/store/mgva7bgb9zi6pnzxii9gvz4sbhs73vj9-roboto-2.138
- [= ] /nix/store/3cg59xc331qda1f7ganxsc7agfp5xcbm-noto-fonts-20201206-phase3
  [  ] /nix/store/xpwj2fxvx6riq8gk2w0xm99pqv9zsk8p-hack-font-3.003
  [  ] /nix/store/kris0s9y6sbis0ma3zjbmqch4kpmryjv-font-cursor-misc-1.0.3
  [  ] /nix/store/3ccww7zvvmizkwa434m8dk6vik8ax6hj-font-misc-misc-1.1.2
  [  ] /nix/store/6ahjzs5nih9s5wz87zajnkxksymmd9qc-dejavu-fonts-2.37
  [  ] /nix/store/ql3inpz448k01x75v8lzmlswlddxlrr9-freefont-ttf-20120503
  [  ] /nix/store/fa1hr0829mjvqskjv9k2iv5qza28ihgi-gyre-fonts-2.005
- [= ] /nix/store/fqinqima2iw0nkl9zk36v5dygbg57669-liberation-fonts-2.1.0
  [  ] /nix/store/f29d0sxg4svwpvd1pplg0fq7g4qdvrmb-unifont-15.0.01
- [= ] /nix/store/afyid3i7s7jq2gk2l0699znlvhz1csk3-noto-fonts-emoji-2.038
  [  ] /home/kyaru/.local/share/fonts
- [ x] /nix/store/zbr5i1jzfaj29v62g9r9xlibhv8295r6-patchelf-0.15.0/share/fonts
- [ x] /nix/store/zd69n70lka5z4b13dgksjpm85sn00nma-konsole-22.12.3/share/fonts
- [ x] /nix/store/ixivqinb3lqyxqk30dr59vhy6criqiyh-knewstuff-5.104.0/share/fonts
- [ x] /nix/store/7pyjjjy8kd7q4jy0lqf953dgyja5r4sj-knewstuff-5.104.0-bin/share/fonts
- [ x] /nix/store/0c4nnihc11yclcz0kz5d52mn864axcv3-kpackage-5.104.0/share/fonts
- [ x] /nix/store/xva5km33l1ycig0y8wwkzlxmxa02640n-kpackage-5.104.0-bin/share/fonts
- [ x] /nix/store/xr3q2b4046v7ijxnpc9yq927zjysb758-kirigami2-5.104.0/share/fonts
- [ x] /nix/store/my4vcpxfwrplbj25zki12dafykpx0rpi-kpty-5.104.0/share/fonts
- [ x] /nix/store/nnmx8xnrl814h9z0cdk0vg3j0pfg3gph-kparts-5.104.0/share/fonts
- [ x] /nix/store/9cva62x17li82yjpv6xxssc8iy20saib-kparts-5.104.0-bin/share/fonts
- [ x] /nix/store/s7dc99wixsvr4klsb1chahh4n6dkwrw6-knotifyconfig-5.104.0/share/fonts
- [ x] /nix/store/7y2xlzzvchawzxpyp9yzr49hdf39n7dr-kinit-5.104.0/share/fonts
- [ x] /nix/store/i3sn4nxvpdkm0y4vf097a92wz5j3lwwl-kwallet-5.104.0/share/fonts
- [ x] /nix/store/l0lk9rx9hjzc5akavgv87fb0233lvgqg-kwallet-5.104.0-bin/share/fonts
- [ x] /nix/store/jl9xm0j1mibpz942g7z61sr0zf84g6wv-knotifications-5.104.0/share/fonts
- [ x] /nix/store/k4yc7shgk40j0s6pl88zp132immg2pv0-kded-5.104.0/share/fonts
- [ x] /nix/store/005svk9wcwzj5bna3221hr6733qlw852-solid-5.104.0/share/fonts
- [ x] /nix/store/c6201mdbydihbv8920l4ry3kpxck6zf1-kjobwidgets-5.104.0/share/fonts
- [ x] /nix/store/0g1g15m4ip0ra6jqc2czcilywjnih3hy-kjobwidgets-5.104.0-bin/share/fonts
- [ x] /nix/store/v29mgghviq7hr5jgnzjvj1pfr9wjsp6w-kio-5.104.0/share/fonts
- [ x] /nix/store/4hdyj9bc5hsf1ikg7pbyrigb9bx9hy1g-knotifications-5.104.0-bin/share/fonts
- [ x] /nix/store/3s7ib1ij0nb8rmr43hb8gzq6l2rrksrh-phonon-4.11.1/share/fonts
- [ x] /nix/store/glr1pj3fy4xd5q116j4vkr5d3yhmmwxi-kbookmarks-5.104.0/share/fonts
- [ x] /nix/store/4291snh6j4nm6binv7plsa7br9sn51qi-kxmlgui-5.104.0/share/fonts
- [ x] /nix/store/hffc9n9x4hyy7r3a4fsc71419i2n9rn1-ktextwidgets-5.104.0/share/fonts
- [ x] /nix/store/g74hsxbs3cm244xwddh3k18im65y839b-sonnet-5.104.0/share/fonts
- [ x] /nix/store/r3s857lk5p8kga0vrz8rv87xsg8d4zkv-kservice-5.104.0/share/fonts
- [ x] /nix/store/pdv3bi48c5s9if8mlys185li5qchdp8n-kservice-5.104.0-bin/share/fonts
- [ x] /nix/store/v9m5n90qnpcr1j45bqirsjlpdadlw9bi-kdbusaddons-5.104.0/share/fonts
- [ x] /nix/store/gnaz2236r2mgih90f900jc6z3kzw96gf-kiconthemes-5.104.0/share/fonts
- [ x] /nix/store/mj21hinvsma9sq7h4bwl5qg6xrb8fx4a-kguiaddons-5.104.0/share/fonts
- [ x] /nix/store/g4p0llkvgh2gc2dhgnsdqpr4s45xzb60-kcoreaddons-5.104.0/share/fonts
- [ x] /nix/store/91rfw3aglaxzvmsrb9vbkpy3qvahwvrk-kcoreaddons-5.104.0-bin/share/fonts
- [ x] /nix/store/0yiqfv8k6asxywz43mlmir5ry25gmy33-kconfig-5.104.0/share/fonts
- [ x] /nix/store/mqfiav0dcy0fxj4n3kszzcnblz3hq0yb-kcodecs-5.104.0/share/fonts
- [ x] /nix/store/v66cdv70wf9b3axnh60a6q5g649crgsq-kcompletion-5.104.0/share/fonts
- [ x] /nix/store/7c7k9pyp213bsr1ywwqbhmxrcbfiikx3-kglobalaccel-5.104.0/share/fonts
- [ x] /nix/store/r3cmn7rpr05sl41m1gsv2srzyj3vsj7b-kwindowsystem-5.104.0/share/fonts
- [ x] /nix/store/582vama8h4qb7qy6pqkpmbcnkdyacbx2-kitemviews-5.104.0/share/fonts
- [ x] /nix/store/ss8vd3srn0sf0kl0v2kq1q3zxazgrhiw-kconfigwidgets-5.104.0/share/fonts
- [ x] /nix/store/aiy0fip4c0dwmc09zq14nn34hs0j32b5-kauth-5.104.0/share/fonts
- [ x] /nix/store/gv8dgzwqrb6x4bqpmqmikp0xkyci7sdl-kwidgetsaddons-5.104.0/share/fonts
- [ x] /nix/store/nsxri3zj3sgbiyqr8kyasq6qjjs7hd0z-flex-2.6.4/share/fonts
- [ x] /nix/store/2ycvidhlvlb5whpy537l7dp0fj5y16cf-gnum4-1.4.19/share/fonts
- [ x] /nix/store/bnmskrxrr0xc1plz2j2nhkj9y3apdw8g-bison-3.8.2/share/fonts
- [ x] /nix/store/kscglda1bvqk4idk6a2ijagpxgz2q6w0-kdoctools-5.104.0/share/fonts
- [ x] /nix/store/zw0jnl4d6k58mzzh8fk2j6xv7vinfmx0-ki18n-5.104.0/share/fonts
- [ x] /nix/store/qnmafi99rm0p8bbqlc1m4xm1vmq3fjq9-karchive-5.104.0/share/fonts
- [ x] /nix/store/w2sssdzzp5ad25pzr1d4cwx5gff5pri1-fontconfig-2.14.0-lib/share/fonts
- [ x] /nix/store/hhp5rgz71srhcpfh9l3kxal5yh0k4bd4-systemd-252.5/share/fonts
- [ x] /nix/store/4x515ma88q96phhjyr8y56im6y5lgz83-glib-2.74.5/share/fonts
- [ x] /nix/store/sc4kxkxb8xd7aani6abbih40lsyss963-gettext-0.21/share/fonts
- [ x] /nix/store/181c2rlnm5qkpx49map3kjvfaibgcjdz-dbus-1.14.4/share/fonts
- [ x] /nix/store/ghcls9l3bvlpyw46gx09bbw3arx5isac-kde-cli-tools-5.27.3/share/fonts
- [ x] /nix/store/j9wsq9j5jqxdwkjqhd3mh3yv2l6j8y7f-plasma-workspace-5.27.3/share/fonts
- [ x] /nix/store/i0g57hyzdjjq6yc2pq5v32pzj74pak35-networkmanager-1.40.12/share/fonts
- [ x] /nix/store/nr0a7w1pw0bpm84a04mpyz98x0zs5vb4-gnutls-3.8.0/share/fonts
- [ x] /nix/store/rl3djm8gb7xkrl5bbz0walri21aaismc-libksysguard-5.27.3/share/fonts
- [ x] /nix/store/r6qrqd3s2l05a0vmrpdqdiq89h9llgch-kxmlrpcclient-5.104.0/share/fonts
- [ x] /nix/store/dz177p3c3y65z8n2j5br6mcpjdn4fa0r-breeze-qt5-5.27.3/share/fonts
- [ x] /nix/store/z44asc8idppyd8jz11c4yidiydmxcawa-kdecoration-5.27.3/share/fonts
- [ x] /nix/store/5c6ks120gd3xp8j3hf2zmk8nas8qsnsw-ktexteditor-5.104.0/share/fonts
- [ x] /nix/store/rwdwb127s94sgrwrca0y070ysdykdqls-syntax-highlighting-5.104.0/share/fonts
- [ x] /nix/store/i76n4nh6cy7bdgxzhkxnc23hbrhsfnix-plasma-framework-5.104.0/share/fonts
- [ x] /nix/store/i7mzm9pxw42hxkv6p1d0w8frf2czqli0-kpeople-5.104.0/share/fonts
- [ x] /nix/store/fgbpmcrs38hipihx7xiba42hslv26rg2-knotifyconfig-5.104.0/share/fonts
- [ x] /nix/store/0r8dhdnz3vakc5ipycn873ghp1q5wq73-knewstuff-5.104.0/share/fonts
- [ x] /nix/store/jwcf5dcyxxxqvj7yzjy66i1sfillvcw6-kjsembed-5.104.0/share/fonts
- [ x] /nix/store/rxqamnbpw3gd3j0nscj95i6r13ipj1fr-kunitconversion-5.104.0/share/fonts
- [ x] /nix/store/p7319ncgf0i8skgnvqpv3hmw8sr5094i-kparts-5.104.0/share/fonts
- [ x] /nix/store/fsmlidrmpjwrx827qrkwz9s9brv7q8a1-kparts-5.104.0-bin/share/fonts
- [ x] /nix/store/rfc9icipbzzjyf7h31f2w9cn3sczg9af-kdesignerplugin-5.104.0/share/fonts
- [ x] /nix/store/0zvfhbg9jpdap6wh6rxva8zcfyxm3y84-kfilemetadata-5.104.0/share/fonts
- [ x] /nix/store/8d1n3148r1w24ba4snm2zwf31fmr0wxl-kpipewire-5.27.3/share/fonts
- [ x] /nix/store/xly29lqnm6ihk8l1n90fw7wggbpdcbx5-kholidays-5.104.0/share/fonts
- [ x] /nix/store/d5p5h3izsiz7d4bp3jkvnb428ar02bx6-libksysguard-5.27.3-bin/share/fonts
- [ x] /nix/store/gmflf5a1wdrs3n6l0kb90ini66ppcrmd-kwin-5.27.3/share/fonts
- [ x] /nix/store/1lizcf0msh6nkbfmy8wlcg85jplyz8lw-breeze-qt5-5.27.3-bin/share/fonts
- [ x] /nix/store/5663acadqva5acdmc4nygva5k92g8nms-frameworkintegration-5.104.0-bin/share/fonts
- [ x] /nix/store/p7h7864scjfcj06mpayq22wki0pd10mq-ktexteditor-5.104.0-bin/share/fonts
- [ x] /nix/store/abi9c5fc4x39c7ha7d76yd48hjh82yfp-kscreenlocker-5.27.3/share/fonts
- [ x] /nix/store/axvj14wp18bmzs6n8qyx110lihwxhsrq-libkscreen-5.27.3/share/fonts
- [ x] /nix/store/pkhc640h893jl07z8mh454x97vrakv0i-krunner-5.104.0-bin/share/fonts
- [ x] /nix/store/w7n1crvjd6hj8qvl2gzyf7ksiv86wbis-plasma-framework-5.104.0-bin/share/fonts
- [ x] /nix/store/s75gj1nxq1scb1i8las122gwdjnblbw5-knewstuff-5.104.0-bin/share/fonts
- [ x] /nix/store/mxsjm0bkcq82sfj637v17jxs5hr65n2i-kdelibs4support-5.104.0/share/fonts
- [ x] /nix/store/9sn0az87drdgficdhmqwkw71k58nfrhc-kemoticons-5.104.0-bin/share/fonts
- [ x] /nix/store/77dmhycqwzbp64fx491xnzzqknf83p13-baloo-5.104.0/share/fonts
- [ x] /nix/store/akg5lfhrlk7zflimqrkrkwj822f2gkf7-kinit-5.104.0/share/fonts
- [ x] /nix/store/iqxxpzawipz8zqwhxi1bi092lgsspgjn-kdesu-5.104.0/share/fonts
- [ x] /nix/store/6qh946z6bj68rcvf22dr9705agrpaj5b-kcmutils-5.104.0/share/fonts
- [ x] /nix/store/cz6grxpgzln219n3v111m683q0gyaqxs-kcmutils-5.104.0-bin/share/fonts
- [ x] /nix/store/pvkqnig10sgg427619rsymkbs9aamnff-kdeclarative-5.104.0/share/fonts
- [ x] /nix/store/x3fxa1qfgs2rbbizlhy9wbjqsysgjkhb-kwallet-5.104.0/share/fonts
- [ x] /nix/store/nlfpm7ljh8igpcxphxcmdmc4r1kdnb2i-kwallet-5.104.0-bin/share/fonts
- [ x] /nix/store/48da65587hy07yy3myizx1lsc6jqxk6v-kbookmarks-5.104.0/share/fonts
- [ x] /nix/store/1rwz6lzmxizrm3wzix0hksv6lfpr06n8-kxmlgui-5.104.0/share/fonts
- [ x] /nix/store/vf6fgk7vfc24k972skvgbsvgxcgc8bii-ktextwidgets-5.104.0/share/fonts
- [ x] /nix/store/2gi2vaa26hbnhmiv3frigc2x7zc8hd2i-kiconthemes-5.104.0/share/fonts
- [ x] /nix/store/nxlpy30mqysnmvxzz8pk6kyzgx0rq6bf-kio-5.104.0/share/fonts
- [ x] /nix/store/wcp1549xmzjnm6ay2zbl2i3wkv8qvg8g-kconfigwidgets-5.104.0/share/fonts
- [ x] /nix/store/id24by5df35sj4zvvfazss8c6bp3fdkb-kauth-5.104.0/share/fonts
- [ x] /nix/store/2wl8zlj4295g1fdy86sa1q1lig084avw-hunspell-1.7.2/share/fonts
- [ x] /nix/store/w2vy58hjnqv8x3s62sk21rrynar8dh9w-pipewire-0.3.66-lib/share/fonts
- [ x] /nix/store/pcslyy22s9piz2n3pckqia0k5i4ysi12-attr-2.5.1/share/fonts
- [ x] /nix/store/inzbkps4dv01p0c5rzdiqp1lzci73kj6-xz-5.4.1/share/fonts
- [ x] /nix/store/3nmcd2i6vdfmm9jzwq6rxsjhgw5lkhib-exiv2-0.27.6/share/fonts
- [ x] /nix/store/a758mcfzpzwwasxfrrg2ff4qyrlqnqk3-appstream-qt-0.15.5/share/fonts
- [ x] /nix/store/3dfdapzh0p6lqla1d61iv7iivj5zq8z3-libqalculate-4.6.0/share/fonts
- [ x] /nix/store/n0lp79i3ac8r4xbbww2sz8qccixs1w9s-linux-pam-1.5.2/share/fonts
- [ x] /nix/store/pq0x1wv67748c4frm493kjm4ryrb243w-iso-codes-4.11.0/share/fonts
- [ x] /nix/store/xnjiqpf036dcs660n8a2qrxldnq7x4s9-desktops/share/fonts
- [ x] /etc/profiles/per-user/kyaru/share/fonts
- [ x] /run/current-system/sw/share/fonts
  [  ] /nix/store/x9cii4yd4c5b3p9k187aysmv2x5dqrlg-dejavu-fonts-minimal-2.37
  [ x] /home/kyaru/.nix-profile/lib/X11/fonts
  [  ] /home/kyaru/.nix-profile/share/fonts
  [ x] /usr/share/fonts
  [ x] /usr/local/share/fonts
  [ x] /nix/var/nix/profiles/default/lib/X11/fonts
  [ x] /nix/var/nix/profiles/default/share/fonts
```
