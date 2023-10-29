---
title: "Privacy Basics"
date: 2023-10-07T09:29:09-03:00
tags: ["privacy", "cryptography", "bitcoin", "cypherpunk"]
categories: []
javascript: false
math: false
mermaid: false
---

## Why Privacy is Important?

First, the definition of privacy according to the [Cyphepunk Manifesto](https://nakamotoinstitute.org/cypherpunk-manifesto):

> Privacy is necessary for an open society in the electronic age.
> Privacy is not secrecy.
> A private matter is something one doesn't want the whole world to know,
> but a secret matter is something one doesn't want anybody to know.
> **Privacy is the power to selectively reveal oneself to the world**.

Privacy is not just a personal preference
but a **fundamental right that is essential for protecting individual freedom,
democracy, and the integrity of the digital world**.

Privacy is also important because it allows individuals to express themselves freely without fear of reprisal.
In a world where dissent is not tolerated,
privacy allows people to speak out against the system without putting themselves in danger.
It also allows people to explore their identities and beliefs without fear of judgment or persecution.

Furthermore, privacy is essential for maintaining trust in relationships.
In a world where everything is monitored and recorded,
privacy is the only way to maintain intimacy and trust in personal relationships.
Without privacy, relationships become transactional and superficial,
devoid of the emotional connection that makes them meaningful.

Finally, privacy is important for democracy.
In a world where information is power,
privacy ensures that everyone has access to the same information and is not unfairly disadvantaged.
Without privacy, those in power can use their access to information to manipulate elections and undermine democracy.

## How to migrate to a secure and private tools?

> NOTE: most of the recommendations here are in sync with the recommendations of [privacyguides.org](https://www.privacyguides.org/).
> I highly recommend their recommendations and guides.

1. Get a Linux-capable notebook.
   [Framework Laptops](https://frame.work/) seems a nice addition with the user customization,
   upgradable and repairable.
   [System76](https://system76.com/) hardware is also great with the open firmware,
   native coreboot and Intel's Management Engine (IME)
   (avoid AMD machines since System76 cannot disable
   the ADM Platform Security Processor - PSP.)
   Also don't forget to disable Intel's ME with [`corna/me_cleaner`](https://github.com/corna/me_cleaner).
1. Get a Google Pixel smartphone.
   Install [GrapheneOS](https://grapheneos.org/).
   It is more secure than any Linux distribution.

[GrapheneOS](https://www.privacyguides.org/android/#grapheneos)
should be your default high-secure medium,
and Linux coming close as a second-best alternative.

## Linux/Qubes

Linux is _not_ a silver bullet.

Linux is [**_not_ safe**](https://madaidans-insecurities.github.io/linux.html).

That's why I recommend [**Qubes OS**](https://www.qubes-os.org/).

### [Qubes OS](https://www.qubes-os.org/)

This is the **most secure persistent-storage OS**.
It uses level-1 virtualization to compartmentalize everything.
There's a great guide on how to install Qubes at [anonymousplanet.org](https://anonymousplanet.org/guide.html#the-qubes-route).
For the extra paranoid there's a [detached encrypted boot and header guide](https://forum.qubes-os.org/t/playing-with-qubes-part-2/12102).
By using a detached header the encrypted blockdevice itself only carries encrypted data,
which gives [deniable encryption](https://en.wikipedia.org/wiki/Deniable_encryption) as long as the existence of a header is unknown to the attackers.
**Deniable encryption** describes encryption techniques where the existence of an encrypted file or message is deniable in the sense that an adversary cannot prove that the plaintext data exists

Some suggestions on how to run your Qubes OS:

1. **Disable Bluetooth.**
   Disable, preferably in BIOS, bluetooth, fingerprint scanner,
   and any other useless stuff that enhances your attack surface.

   Alternatively, you can start `dom0` and edit the `/etc/modprobe.d/blacklist-bluetooth.conf`:

   ```bash
   blacklist bnep
   blacklist bluetooth
   blacklist btusb
   ```

1. **Activate [Anti Evil Maid](https://github.com/QubesOS/qubes-antievilmaid)**,
   if your hardware supports it (TPM 1.2).
1. **Use Btrfs filesystem**.
   The default Qubes filesystem is LVM-enabled ext4.
   If you have very large (in space) VMs you might [experience issues](https://github.com/QubesOS/qubes-issues/issues/5426#issuecomment-678740371).
   `dom0` really hogs up the cpu after shutdown of huge app VMs.
   Btrfs solves this issue.
1. **Split Everything**:

   - **Split GPG**: Follow the instructions at [`qubes-os.org/doc/split-gpg`](https://www.qubes-os.org/doc/split-gpg/)
   - **Split SSH**: Follow the instructions at [`Qubes-Community` Docs](https://github.com/Qubes-Community/Contents/blob/master/docs/configuration/split-ssh.md)
   - **U2F Proxy**: Follow the instructions at [`qubes-os.org/doc/u2f-proxy`](https://www.qubes-os.org/doc/u2f-proxy/)

### Linux

> Don't forget to check my [NixOS paranoid build with root on `tmpfs`](https://github.com/realeinherjar/flakes).
> This means that everything outside some directories of `/etc` and `/home` will be wiped out.

Knowing that if you still want to use Linux, go ahead.
Linux is much better than Windows.
There are some nice information on [how to harden your Linux system](https://madaidans-insecurities.github.io/guides/linux-hardening.html) (additional resource [here](https://privsec.dev/posts/linux/desktop-linux-hardening/))
If you want the maximum available privacy and security in your Linux distribution:

1. **Use Private and Secure Distributions!**
   Avoid telemetry that often comes with proprietary operating systems.
   Maintain [software freedom](https://www.gnu.org/philosophy/free-sw.en.html#four-freedoms).
   Encrypt your system (**ALWAYS**).
1. [**Use Secure Boot**](https://www.privacyguides.org/linux-desktop/hardening/#secure-boot).
   [Secure Boot](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface#Secure_Boot) can be used to secure the boot process by preventing the loading of unsigned UEFI drivers or boot loaders.
   This is specially useful against [evil maid attacks](https://en.wikipedia.org/wiki/Evil_maid_attack).
1. **Don't use Swap partition.**
   Consider using [ZRAM](https://docs.fedoraproject.org/en-US/fedora-coreos/sysconfig-configure-swaponzram/) to avoid potential security issues with sensitive data being pushed to swap space.
1. **Use [Wayland](https://wayland.freedesktop.org/).**
   It is [more secure than Xorg](https://www.privacyguides.org/linux-desktop/#wayland),
   because it is developed with [security in mind](https://lwn.net/Articles/589147/),
   supporting GUI isolation,
   allowing none of the windows to record screen,
   log and inject inputs in other window.
1. **Use [Network Manager MAC randomization](https://fedoramagazine.org/randomize-mac-address-nm/).**
   Create a file `/etc/NetworkManager/conf.d/wifi_rand_mac.conf`:

   ```bash
   [device-mac-randomization]
   # "yes" is already the default for scanning
   wifi.scan-rand-mac-address=yes

   [connection-mac-randomization]
   # Randomize MAC for every ethernet connection
   ethernet.cloned-mac-address=random
   # Generate a randomized value upon each connection
   wifi.cloned-mac-address=random
   # Generate a random MAC for each WiFi and associate the two permanently
   #wifi.cloned-mac-address=stable
   ```

1. **Use a `/etc/hosts` file to block spyware, adware and malware system-wide.**
   I recommend [`StevenBlack/hosts`](https://github.com/StevenBlack/hosts).
1. **If you need [VPN](https://madaidans-insecurities.github.io/vpns.html),
   use [mullvad](https://mullvad.net),
   or [IVPN](https://www.ivpn.net)**
1. **Use a custom [DNS Resolver](https://www.privacyguides.org/dns/).**
   I sometimes use [mullvad's adblock DNS over TLS](https://mullvad.net/en/help/dns-over-https-and-dns-over-tls/):
   `adblock.doh.mullvad.net`;
   or [adguard's adblock DNS over TLS](https://adguard-dns.io/en/public-dns.html): `dns.adguard-dns.com`.
   You can find the list at [`mullvad/dns-blocklists`](https://github.com/mullvad/dns-blocklists/tree/main/lists).
1. **Use a [firewall](https://www.privacyguides.org/linux-desktop/hardening/#firewalls).**
   You could also set your default firewall zone to drop packets:

   ```bash
   firewall-cmd --set-default-zone=drop;
   firewall-cmd --add-protocol=ipv6-icmp --permanent;
   firewall-cmd --add-service=dhcpv6-client --permanent;
   ```

1. **Use [Full Disk Encryption](https://fedoraproject.org/wiki/Disk_Encryption_User_Guide).**
   Also make sure that you PBKDF is `argon2i` as [detailed below](#a-note-about-quantum-computers).
1. **Secure your [bootloader (generally GRUB) with a password](https://docs.fedoraproject.org/en-US/fedora-coreos/grub-password/).**
1. **Never leave your master GPG key in your computer.**
   The files in our home directory are not as well protected as we like to think.
   They can be leaked or stolen.
   Remove your master key from your home directory and store it on offline storage.
   Use subkeys.
   Follow the [Linux Kernel Maintainer PGP Guide](https://www.kernel.org/doc/html/latest/process/maintainer-pgp-guide.html),
   and also the [Riseup OpenPGP Best Practices](https://help.riseup.net/en/security/message-security/openpgp/best-practices)
   (which are at
   [Jacob Appelbaum’s `duraconf` “collection of hardened configuration files”](https://github.com/ioerror/duraconf)).
   This means getting a PGP Smart card.
   Yubikey are prevalent all over the world,
   and you can have easy access to buying them.
   Use it.
1. **If you are accessing through SSH,
   [turn on a stronger authentication based on public key encryption](https://wiki.gentoo.org/wiki/Security_Handbook/Securing_services#SSH).**
1. **[Sandbox](https://www.privacyguides.org/linux-desktop/sandboxing/#mandatory-access-control) almost everything that you don't trust.**
   If you need to use proprietary crap,
   run in flatpak and use [`flatseal`](https://github.com/tchx84/flatseal)
   for a simple GUI to sandbox permissions.
1. **Disable Bluetooth**:
   You can disable at OS level with `systemd` and at kernel level with `modprobe`.

   - `systemd`:

     ```bash
     systemctl stop bluetooth.service
     systemctl disable bluetooth.service
     systemctl mask bluetooth.service
     ```

   - `modprobe`:
     You may instead want to tell your kernel not to load the driver for these wireless devices at all.
     This means your OS and kernel won't know how to interface with these devices, and they will remain unpowered.

     First, check what kernel modules are currently loaded with `lsmod`:

     ```bash
     cat@rt~ $ sudo lsmod
     Module                  Size  Used by
     btusb                  57344  0
     btrtl                  20480  1 btusb
     btbcm                  16384  1 btusb
     btintel                28672  1 btusb
     bluetooth             577536  5 btrtl,btintel,btbcm,btusb
     ecdh_generic           16384  1 bluetooth
     ecc                    32768  1 ecdh_generic
     [...]
     ```

     We can see on my machine there are several bluetooth drivers running.
     `btusb`, `btrtl`, `btbcm`, `btintel`, and `bluetooth`.

     `btusb` is the generic driver that each of the other modules relies on.
     It should be sufficient just to unload `btusb`,
     but since I know the others aren't going to be used either,
     I like to be thorough and make sure none of these drivers load.

     We will do this with `modprobe` by telling its configuration file to ignore
     these kernel modules with the `blacklist` command.
     This will go in `/etc/modprobe.d/blacklist.conf`:

     ```bash
     blacklist btusb
     blacklist btrtl
     blacklist btbcm
     blacklist btintel
     blacklist bluetooth
     ```

     To avoid GNOME crashes install the `gnome-shell-extension-remove-bluetooth-icon` package.

### Open Source Alternatives

Here are some of my suggestions for you to migrate your proprietary tools to open source tools:

- Messaging: [Signal](https://signal.org/)
  (or the FOSS fork [Molly](https://molly.im/)),
  [SimpleX](https://simplex.chat/),
  [Element (Matrix client)](https://element.io/),
  [Session](https://getsession.org/),
  [Briar](https://briarproject.org/)
  and [Cwtch](https://cwtch.im/).
- Email: [ProtonMail](https://protonmail.com/)
  or [Tutanota](https://tutanota.com/)
  with [NeoMutt](https://neomutt.org/) or [aerc](https://aerc-mail.org/);
  depending on your threat model you might want to self-host your mail server,
  try [`LukeSmithxyz/emailwiz`](https://github.com/LukeSmithxyz/emailwiz).
  You can also try GUI interfaces like [Thunderbird](https://www.thunderbird.net),
  or the fine-tuned fork [Betterbird](https://www.betterbird.eu/).
- Conferencing: [Jami](https://jami.net/) and [Jitsi](https://meet.jit.si/).
- Text Editing: [Helix](https://helix-editor.com/)/[NeoVim](https://neovim.io/)/[Vim](https://www.vim.org)/[VSCodium](https://github.com/VSCodium/vscodium)
  for Markdown, text and LaTeX,
  [LibreOffice](https://www.libreoffice.org/) and [Pandoc](https://pandoc.org/).
- PDF Documents: [Zathura](https://pwmt.org/projects/zathura/).
  (available in every Linux major distro),
  [Sioyek](https://github.com/ahrm/sioyek)
  (supports highlighting and other fancy nice stuff),
  and [Evince](https://wiki.gnome.org/Apps/Evince) (which comes by default in GNOME).
- Ebooks: [Calibre](https://calibre-ebook.com/) and
  [Foliate](https://johnfactotum.github.io/foliate/).
- Web Browser: [hardened Firefox](https://github.com/arkenfox/user.js),
  [Mullvad Browser](https://mullvad.net/browser),
  and [ungoogled-chromium](https://github.com/ungoogled-software/ungoogled-chromium) if you need a Chromium-based browser.
  You can get it using [flatpak](https://flatpak.org/) in [flathub](https://flathub.org/apps/details/com.github.Eloston.UngoogledChromium).
- Search: [searX](https://searx.space/).
  You can use the [Random searX Redirector](https://searx.neocities.org/),
  that will forward your search to one of the 75 random volunteer-run public searX servers to thwart mass surveillance.
- News: You can get RSS feeds with [NewsFlash](https://gitlab.com/news-flash/news_flash_gtk) (GUI)
  or [newsboat](https://newsboat.org/) (CLI).
- Password Manager: always go OFFLINE either [KeePassXC](https://keepassxc.org/)
  or [pass](https://www.passwordstore.org/).
- Cloud: backup stuff with [rsync](https://rsync.samba.org/) and
  [clonezilla](https://clonezilla.org/);
  always encrypt stuff with [LUKS](https://gitlab.com/cryptsetup/cryptsetup/),
  [VeraCrypt](https://www.veracrypt.fr/) (VeraCrypt has [plausible deniability](https://veracrypt.fr/en/Plausible%20Deniability.html)),
  or [Cryptomator](https://cryptomator.org).
- File Sharing: [onionshare](https://onionshare.org/),
  [securedrop](https://securedrop.org/)
  and [croc](https://github.com/schollz/croc).
- Image Editing: [Inkscape](https://inkscape.org/) for vector graphics,
  or [GIMP](https://www.gimp.org/) for image editing.
- Video: [OBS Studio](https://obsproject.com/) for video capturing,
  [ffmpeg](https://ffmpeg.org/) for CLI video editing, or
  [Shotcut](https://www.shotcut.org/) for GUI video editing.
- Music Player: [Cmus](https://cmus.github.io/)
  for a CLI app,
  or [Lollypop](https://wiki.gnome.org/Apps/Lollypop)
  for a GUI app.
- Spotify: DON'T! Use `mp3` files (or even better `opus` files),
  but if you want,
  [SpotiFlyer](https://github.com/Shabinder/SpotiFlyer),
  [SpotDL](https://github.com/spotDL/spotify-downloader),
  and [spotify-adblock](https://github.com/abba23/spotify-adblock).
  There's also [Spotube](https://github.com/KRTirtho/spotube).
- YouTube: [FreeTube](https://github.com/FreeTubeApp/FreeTube),
  [NewPipe](https://github.com/TeamNewPipe/NewPipe),
  [youtube-dl(deprecated)](https://github.com/ytdl-org/youtube-dl),
  [yt-dlp](https://github.com/yt-dlp/yt-dlp)
  and [ytfzf](https://github.com/pystardust/ytfzf).
- Netflix etc: Torrents
  ([transmission](https://github.com/transmission/transmission))
  with [vlc](https://github.com/videolan/vlc) and
  [mpv](https://mpv.io/)
  (also try [peerflix](https://github.com/mafintosh/peerflix) or [webtorrent-cli](https://github.com/webtorrent/webtorrent-cli)).
- IDE: [Helix](https://helix-editor.com/)/[NeoVim](https://neovim.io/) with
  [LSP](https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md),
  or [VSCodium](https://github.com/VSCodium/vscodium).
- Reference Manager: [Zotero](https://www.zotero.org/) (GUI)
  or [papis](https://papis.readthedocs.io/) (CLI).

## GrapheneOS

[GrapheneOS](https://grapheneos.org/) is a privacy and security focused mobile OS
with Android app compatibility developed as a non-profit open source project.

GrapheneOS improves the privacy and security of the OS from the bottom up.
It deploys technologies to mitigate whole classes of vulnerabilities and
make exploiting the most common sources of vulnerabilities substantially more difficult.
It improves the security of both the OS and the apps running on it.
The app sandbox and other security boundaries are fortified.
GrapheneOS tries to avoid impacting the user experience
with the privacy and security features.
Ideally, the features can be designed so that they're always enabled with
no impact on the user experience and
no additional complexity like configuration options.
It's not always feasible, and GrapheneOS does add various toggles for features
like the Network permission,
Sensors permission, restrictions when the device is locked (USB peripherals,
camera, quick tiles), etc. along with more complex user-facing privacy
and security features with their own UX.

GrapheneOS is recommended as one of the [most secure and private option on Android](https://madaidans-insecurities.github.io/android.html).

### How to install

GrapheneOS has two officially supported installation methods.
You can either use the [WebUSB-based installed](https://grapheneos.org/install/web)
recommended for most users
or the [command-line installation guide](https://grapheneos.org/install/cli)
aimed at more technical users.

### A note regarding battery life

You don't need to do battery optimizations.
Modern phones with built-in batteries have their own computers onboard the computer,
which can take into account the number of charge cycles the battery has undergone and automatically reallocate overprovisioning as required to ensure they automatically and transparently set the charging and cutoff thresholds to give the best health of the battery over time.
This makes the old school of thought of charging to 80% obsolete.

Take a look at the [GrapheneOS battery recommendations](https://gist.github.com/Peter-Easton/4982f66e93387e02dd2c1d677d71f4f2)

### Apps

It already comes with a nice browser
([Vanadium](https://github.com/GrapheneOS/Vanadium)) and a good Camera App.

You can also [sandbox Google Play apps in GrapheneOS](https://grapheneos.org/usage#sandboxed-google-play)

The vital apps to have:

> A good list can be found at [`brainfucksec.github.io`](https://brainfucksec.github.io/android-foss-apps-list).

1. [F-Droid](https://f-droid.org).
1. [Aurora Store for Google Play Apps](https://f-droid.org/packages/com.aurora.store):
   - Banking Apps (You'll probably need to [sandbox](https://grapheneos.org/usage#sandboxed-google-play)
     them in a separated user).
     Also for those nasty banking apps you can check one of the [GrapheneOS' core dev banking apps compatibility page](https://privsec.dev/apps/banking-applications-compatibility-with-graphheneos/).
   - Uber (You don't need the app, `m.uber.com` in a mobile browser works great)
1. [Emails with FairEmail](https://f-droid.org/packages/eu.faircode.email/)
   It supports PGP sign/encrypt.
   - You will need to use the [GitHub version](https://github.com/M66B/FairEmail)
     to support Gmail.
1. [Privacy Browser with Bromite](https://www.bromite.org/fdroid)
   (additionally check [`brainfucksec.github.io`](https://brainfucksec.github.io/bromite-android-browser) for Bromite hardening tips)
1. [Calendar with DAVx5](https://f-droid.org/packages/at.bitfire.davdroid/):
   - [Google](https://www.davx5.com/tested-with/google)
1. Passwords and TOPT with [KeePassDX](https://f-droid.org/packages/com.kunzisoft.keepass.libre).
1. GPG Keys with [OpenKeyChain](https://f-droid.org/packages/org.sufficientlysecure.keychain/).
1. If you need [VPN](https://madaidans-insecurities.github.io/vpns.html)
   use [mullvad](https://f-droid.org/packages/net.mullvad.mullvadvpn/),
   or [IVPN](https://www.ivpn.net/).
1. [Email alises with SimpleLogin](https://f-droid.org/packages/io.simplelogin.android.fdroid/).
1. [YouTube with NewPipe](https://f-droid.org/packages/org.schabi.newpipe/).
1. [Spotify with SpotiFlyer](https://f-droid.org/packages/com.shabinder.spotiflyer/).
   Note that you can also download Spotify playlists with [SpotDL](https://github.com/spotDL/spotify-downloader).
   There's an open source frontend for Spotify called [Spotube](https://f-droid.org/packages/oss.krtirtho.spotube/).
   There's an open source frontend for YouTube Music called [ViMusic](https://f-droid.org/packages/it.vfsfitvnm.vimusic/).
1. Videos with [VLC](https://f-droid.org/packages/org.videolan.vlc/) or [mpv](https://f-droid.org/packages/is.xyz.mpv/).
1. [RSS Feeds with Feeder](https://f-droid.org/packages/com.nononsenseapps.feeder/).
1. [PDF Scanner with Open Note Scanner](https://f-droid.org/packages/com.todobom.opennotescanner).
1. [TorBrowser with Guardian Project's F-Droid Repository](https://support.torproject.org/tormobile/tormobile-7).
1. [Navigation with OsmAnd+](https://f-droid.org/packages/net.osmand.plus) and [Organic Maps](https://f-droid.org/packages/app.organicmaps/).
1. Messaging with [Signal](https://signal.org/android/apk/),
   [Session](https://fdroid.getsession.org/),
   and [SimpleX](https://f-droid.org/packages/chat.simplex.app/).
1. Removing EXIF and compressing Images with [ImagePipe](https://f-droid.org/packages/de.kaffeemitkoffein.imagepipe),
   [ObscuraCam](https://guardianproject.info/apps/obscuracam/)
   or [Scrambled Exif](https://f-droid.org/packages/com.jarsilio.android.scrambledeggsif/).
1. [Weather with Forecastie](https://f-droid.org/packages/cz.martykan.forecastie/).
1. [Online Meetings with Jami](https://f-droid.org/packages/cx.ring/).
1. [PDF and ebooks with Librera Reader](https://f-droid.org/packages/com.foobnix.pro.pdf.reader/).
1. Torrents and General downloads with [Aria2App](https://f-droid.org/packages/com.gianlu.aria2app/)
   [LibreTorrent](https://f-droid.org/packages/org.proninyaroslav.libretorrent/).
1. [FTPClient](https://f-droid.org/packages/de.qwerty287.ftpclient/).
1. [Control your Smartphone Internet with NetGuard](https://f-droid.org/packages/eu.faircode.netguard/).
   You don't need to install this.
   The most effective way to block network access is to revoke the network permission from the app like [GrapheneOS allows you to do](https://grapheneos.org/features#network-permission-toggle).
1. [Block Adware, Spyware and Malware with AdAway](https://adaway.org/)
   (it uses [`StevenBlack/hosts`](https://github.com/StevenBlack/hosts)
   under the hood).
1. [Block unwanted calls with Yet Another Call Blocker](https://f-droid.org/packages/dummydomain.yetanothercallblocker/).
1. [Simple Mobile Tools](https://www.simplemobiletools.com/).

## What about VPN?

> If you are using a VPN, check if you are leaking your DNS with <https://dnsleaktest.com/>.

VPNs are no [silver bullet](https://www.privacyguides.org/vpn/).
Do not trust all the VPN ads.
They are made for you to **buy** stuff.
You are just transferring the trust from your ISP
(**I**nternet **S**ervice **P**rovider)
to your VPN service provider.

But they are useful, specifically for:

1. You need to have a specific geolocated IP address.
1. You need to hide some stuff from your ISP
   (this is most important if you are a person of interest in
   a f\*\*\*\*\*-up country).
1. You need to increase your anonymity set,
   i.e. have a plausible deniability that you might or might not be someone.
   This is recommended when downloading and seeding torrents.

Suggestions:

- If you want a **simple solution** just use
  [Mullvad](https://mullvad.net/en/)
  or [IVPN](https://www.ivpn.net/).
  They are notorious private VPN services that claims to do no logging and
  also no personal information.
  Also, Mullvad is located in Sweden which has very good privacy-respecting laws.
  Buy a subscription using cryptocurrency with obfuscation techniques,
  such as Monero or coinjoined Bitcoin.
  If you need something quick and dirty,
  IVPN offers [IVPN Light](https://www.ivpn.net/light/)
  which gives you VPN access for 3 hours, 1 day, 1 week, or 1 month.
  You can pay with lightning.
- If you want to **do it yourself** see this [video guide by mental outlaw](https://www.youtube.com/watch?v=szGsh5J9bzY).
  It teaches you how to make your own VPN with OpenBSD
  (a **VERY** secure Unix-based OS) and WireGuard.
  For the VPS hosting, use either [1984](https://1984.hosting/)
  or [orange](https://www.orangewebsite.com/).
  Both are located in Iceland which has very good privacy-respecting laws.
- You can also just use [**Tor**](https://www.torproject.org/) to obfuscate you traffic.

If you need to use a VPN but somehow either obfuscate even further
or bypass some restriction or firewall block,
use [Shadowsocks](https://shadowsocks.org/).

### VPN Setup

> Sources: [manpage of `wg-quick`](https://manpages.debian.org/unstable/wireguard-tools/wg-quick.8.en.html),
> [Mullvad WireGuard on Linux terminal](https://mullvad.net/en/help/easy-wireguard-mullvad-setup-linux/)
> [IVPN Autostart WireGuard in systemd](https://www.ivpn.net/knowledgebase/linux/linux-autostart-wireguard-in-systemd/),
> and [IVPN WireGuard Kill Switch](https://www.ivpn.net/knowledgebase/linux/linux-wireguard-kill-switch/)

For the extra paranoid, you can use VPNs without installing their apps.
You will [WireGuard](https://www.wireguard.com/), which is available in almost all Linux distributions.
Depending on circumstances, just installing `wireguard-tools` will suffice
(where all necessary dependencies will be installed).

1. Create your configuration in `/etc/wireguard/wg0.conf`.
   You can also name `wg0.conf` whatever you want.
   Any free-form string `[a-zA-Z0-9_=+.-]{1,15}` will work.
   These configs are generally provided by your VPN provider.
   They generally look something like this:

   ```shell
   [Interface]
   PrivateKey = abcdefghijklmnopqrstuvwxyz0123456789=
   Address = x.y.z.w/32
   DNS = x.y.z.w
   [Peer]
   PublicKey = abcdefghijklmnopqrstuvwxyz0123456789=
   Endpoint = sub.wg.domain.tld:9999
   AllowedIPs = 0.0.0.0/0
   ```

1. Add "kill switch" configs.
   Add the following two lines to the `[Interface]` section,
   just before the `[Peer]` section:

   ```shell
   PostUp  = iptables -I OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT && ip6tables -I OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT
   PreDown = iptables -D OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT && ip6tables -D OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT
   ```

   You may get a problem to connect to your local network.
   You can modify the kill switch,
   so it includes an exception for your local network,
   for example `! -d 192.168.1.0/24`:

   ```shell
   PostUp  = iptables -I OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL ! -d 192.168.1.0/24 -j REJECT && ip6tables -I OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT
   PreDown = iptables -D OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL ! -d 192.168.1.0/24 -j REJECT && ip6tables -D OUTPUT ! -o %i -m mark ! --mark $(wg show %i fwmark) -m addrtype ! --dst-type LOCAL -j REJECT
   ```
1. Make sure that you have the correct permissions, so only `root` can read them:

   ```bash
   sudo chown root:root -R /etc/wireguard && sudo chmod 600 -R /etc/wireguard
   ```

1. Start the WireGuard connection with:

   ```bash
   sudo wg-quick up wg0
   # to disconnect
   sudo wg-quick down wg0
   ```

#### Autostart WireGuard in `systemd`

If you are using a Linux distribution that comes with `systemd`,
you can autostart a WireGuard connection with:

```bash
sudo systemctl enable wg-quick@wg0.service
sudo systemctl daemon-reload
sudo systemctl start wg-quick@wg0
```

To check status: `sudo systemctl status wg-quick@wg0`

To remove the service and clean up the system:

```bash
sudo systemctl stop wg-quick@wg0
sudo systemctl disable wg-quick@wg0.service
sudo rm -i /etc/systemd/system/wg-quick@wg0*
sudo systemctl daemon-reload
sudo systemctl reset-failed
```

#### Testing the Kill Switch

One way to test a down tunnel is to delete the IP address from the WireGuard network interface,
like this via the Terminal:

```bash
sudo ip a del [IP address] dev [interface]
```

In this example, it’s possible to remove `x.y.z.w` from the `wg0` interface:

```bash
sudo ip a del x.y.z.w/32 dev wg0
```

The `PostUP` iptables rule from above restricts all traffic to the tunnel,
and all outgoing attempts to get traffic out fail.
To gracefully recover from this,
you will likely have to use the `wg-quick` command to take the connection down,
then bring it back up.

## Browser Extensions

Remember that the browser is pretty much a **_glorified_ remote code execution machine**.
That pretty much means a privacy and security nightmare.
So you need to have sane defaults.
The idea here is to block everything (_opt-out_) by default and whitelist (_opt-in_) as necessary.

[uBlock Origin](https://github.com/gorhill/uBlock), the only one you need!
I recommend using the [Hard Mode](https://github.com/gorhill/uBlock/wiki/Blocking-mode:-hard-mode),
with [disabled JavaScript](https://github.com/gorhill/uBlock/wiki/Per-site-switches#no-scripting)
and whitelist sites as necessary.

## Email Aliasing

It is one of the most powerful privacy devices that you have.
I use [simplelogin](https://simplelogin.io).

## File Sharing

**Send** is a fork from the discontinued project Firefox Send.
It provides [end-to-end encryption](https://github.com/timvisee/send/blob/master/docs/encryption.md) with optional password.
You can run your [own instance](https://github.com/timvisee/send-instances/#host-your-own-instance).
The author of this fork also provides a [command-line client](https://github.com/timvisee/ffsend) to send files from your terminal to the Firefox Send instances.

Many [instances](https://github.com/timvisee/send-instances) are available, with different characteristics,
such as more storage or longer available time.

## Backups

It is important to backup your data.
First, a note about **cloud solutions**.
**Stay away!**
Most cloud solutions will scan your data or keep copies of your data for years after deletion.
If you want to backup to the cloud use an **encrypted cloud solution**,
such as [Proton Drive](https://drive.proton.me) or [Cryptee](https://crypt.ee).
Both have good cryptographic standards and zero-knowledge encryption,
but Cryptee has amazing features like no-KYC and plausible deniability,
apart from being hosted on Estonia (privacy-respecting country).

Said that, I highly advocate you do **offline encrypted backups**.
You can use open source and widely-available tools such as [**GnuPG**](https://gnupg.org)/[**VeraCrypt**](https://www.veracrypt.fr/) and [**rsync**](https://rsync.samba.org/).
Or you can use automated open source tools
(that might not be widely-available, i.e. you would need to install manually)
such as [`rbackup`](https://github.com/brainfucksec/rbackup).
`rbackup` is a simple
(with an easy-to-read codebase)
shell script for making backup of your Linux system with `rsync`.
Uses `tar` and `gzip` for compression and GnuPG for encryption.

## Alternative Frontends

There are several FOSS alternative frontends such as
[Nitter](https://github.com/zedeus/nitter),
[Libreddit](https://github.com/libreddit/libreddit),
[Invidious](https://invidious.io/),
[Wikiless](https://github.com/Metastem/wikiless),
[Bibliogram](https://sr.ht/~cadence/bibliogram/),
and [ProxiTok](https://github.com/pablouser1/ProxiTok).
Use them whenever possible for a privacy-oriented consumption of those services.

[Farside](https://sr.ht/~benbusby/farside/) is redirecting service for FOSS alternative frontends.

Farside provides links that automatically redirect to working instances of privacy-oriented alternative frontends, such as Nitter, Libreddit, etc.
This allows for users to have more reliable access to the available public instances for a particular service,
while also helping to distribute traffic more evenly across all instances and avoid performance bottlenecks and rate-limiting.

Farside's links work with the following structure: `farside.link/<service>/<path>`.
Examples:

- `https://farside.link/https://www.youtube.com/watch?v=dQw4w9WgXcQ` will redirect to a [Piped](https://github.com/TeamPiped/Piped) or [Invidious](https://github.com/iv-org/invidious) instance.
- `https://farside.link/reddit.com/r/popular` will redirect to a [Libreddit](https://github.com/spikecodes/libreddit) or [Teddit](https://codeberg.org/teddit/teddit) instance.

## Password Policy

**Always generate strong passwords**.
That means at least [200 bits of entropy for quantum secure](https://www.kicksecure.com/wiki/Passwords).
You can easily generate this with 15 words from the [EFF wordlist](https://www.eff.org/dice).

> NOTE: [KeepassXC](https://keepassxc.org) [comes already with the EFF wordlist](https://keepassxc.org/).

## Cryptography

Encryption of data is the only way to control who can access it.
**Always encrypt sensitive data!**

### A note about Quantum Computers

Quantum computers are a threat to modern cryptographic algorithms and softwares.
[Grover’s algorithm](https://en.wikipedia.org/wiki/Grover%27s_algorithm) shows that a quantum computer speeds up these attacks to [effectively halve the key length](https://www.schneier.com/blog/archives/2018/09/quantum_computi_2.html).
This would mean that a [256-bit key is as strong against a quantum computer as a 128-bit key is against a conventional computer](https://www.kicksecure.com/wiki/Passwords#Classical_vs_Quantum_Computing);
both are secure for the foreseeable future.

> NOTE: Passwords for LUKS FDE are secured using a key-stretching implementation known as a Password-Based Key Derivation Function (PBKDF).
> The older `HMAC-SHA*` algorithm is less effective against parallelization by GPUs and ASICs -- the same weaknesses suffered by Bitcoin against dedicated mining hardware.
> Argon2id is the winner of the [Password Hashing Competition](https://en.wikipedia.org/wiki/Password_Hashing_Competition)
> and the state-of-the-art hashing algorithm of choice when protecting encryption keys.
> Its memory-hard properties introduce a large penalty for brute-search on general computers and even more so on GPUs and ASICs.
> Current versions of LUKS uses `argon2i` as the PBKDF.
> To check this run `sudo cryptsetup luksDump /dev/<luks-device> | grep -E 'Version|cipher|key|PBKDF|Hash'`.

### VeraCrypt

I recommend [**VeraCrypt**](https://www.veracrypt.fr/).
It is open source,
works on Windows/Mac/Linux,
You can encrypt a file, a partition, or a whole storage device.

In case an adversary forces you to reveal your password,
VeraCrypt provides and supports [**plausible deniability**](https://veracrypt.fr/en/Plausible%20Deniability.html).
You can create [**Hidden Volumes**](https://veracrypt.fr/en/Hidden%20Volume.html).
It may happen that you are forced by somebody to reveal the password to an encrypted volume.
There are many situations where you cannot refuse to reveal the password (for example, due to extortion).
Using a so-called hidden volume allows you to solve such situations without revealing the password to your volume.
If you type your default password you unlock the "standard volume",
and if you type another password you unlock the "hidden volume".
The hidden volume is where you would put all of your sensitive information.

### Steganography

[**Steganography**](https://en.wikipedia.org/wiki/Steganography) is the practice of concealing a message within another message or a physical object.
In computing/electronic contexts, a computer file, message, image, or video is concealed within another file, message, image, or video.
This is a very interesting technique where you can easily hide sensitive data into an innocent picture, text file or video.

You have a couple of options for steganography software:

1. [`steghide`](https://github.com/StefanoDeVuono/steghide):
   CLI available in almost every distro (Fedora, Debian, etc.) written in C++.
1. [`tomb`](https://github.com/dyne/Tomb#how-does-it-work):
   a minimalist and easy to use CLI written as a simple shell script (Zsh) using standard filesystem tools (GNU) and the cryptographic API of the Linux kernel (cryptsetup and LUKS).

## Financial Sovereignty with Bitcoin

> If you don't know the case for Bitcoin, check [bitcoiner.guide](https://bitcoiner.guide/).
> For a quick intro on how to use it privately and safely check [bitcoin-intro.com](https://bitcoin-intro.com/).

This list is always evolving.
Check [kycnot.me](https://kycnot.me/).

Regarding Bitcoin, you can:

1. Coinjoin to enhance your on-chain privacy with [Jam](https://jamapp.org/).
1. Use a prepaid Visa card with [Moon](https://paywithmoon.com/).
1. Use a prepaid gift card with [CoinCards](https://coincards.com/).
1. Swap Bitcoin for cash in [Bitcoins ATM](https://coinatmradar.com/).
1. Swap cash for Bitcoin using [Azte.co](https://azte.co/).
   They are accepted in a lot of countries.
   In Brazil, they are accepted in every lotérica.
   All you need is a CPF number.
   There's a 7% flat rate over the current exchange rate for BTC purchases.
   You can withdraw to on-chain (USD 2.5 fee) or lightning (free).
1. P2P decentralized exchange of crypto with [bisq](https://bisq.network/),
   [robosats](https://learn.robosats.com/),
   [localmonero](https://localmonero.co/) or [agoradesk](https://agoradesk.com/).
1. Swap Bitcoin for Monero with [atomic swaps](https://unstoppableswap.net/),
   [fixed float](https://fixedfloat.com/),
   [sideshift](https://sideshift.ai/)
   or [majesticbank](https://majesticbank.sc/);
   see best rates at [orangefren](https://orangefren.com/).

### Wallets

> In case of losing your wallet, you are pretty much f\*\*\*ed.
> You might try your luck, in that case check [`walletsrecovery.org`](https://walletsrecovery.org/).

We have **software wallets** and **hardware wallets**.

Software wallets put your seed into the same device they are running.
So, if your device is compromised, so potentally is your seed.
Your precious cryptocurrencies could be in danger.

Hardware wallets are dedicated hardware to hold,
with tons of security features,
your private keys.
So, if your device is compromised,
your keys are probably safe from danger.
Or at least, way safer than if it were a software wallet.

> If you are not running your own [node](https://node.guide)
> (some reasons why you should do it [here](https://armantheparman.com/why-should-you-run-your-own-bitcoin-node/)),
> you should either use a trusted VPN (which none are) or the Tor network to connect to a trusted node (which again none are).

### Hardware Wallets

The only one I recommend are the ones from [coinkite](https://coinkite.com/) which are Bitcoin-only.
They are very well-designed and almost impossible to breach.

There are two options:

- Higher-end option: [COLDCARD](https://coldcard.com/).
- Cheap option: [TAPSIGNER](https://tapsigner.com/).

### Software Wallets

The best software wallet alternatives are to use Samourai and Monerujo,
for Bitcoin and Monero respectively,
in your GrapheneOS phone.
But you can also use any of these:

1. **Bitcoin**: Sparrow (Linux/MacOS/Windows), Electrum (Linux/MacOS/Windows/Android/iOS),
   BlueWallet (F-Droid/iOS), Nunchuk (Android/iOS),
   Samourai (Android/F-droid),
   or Mutiny (Android/iOS but there's a PWA that runs in the browser).
1. **Monero**: Monero Wallet CLI/GUI (Linux/MacOS/Windows), Cake (Android, iOS),
   Monerujo (F-Droid) or MyMonero (Android/iOS).

## Honorable Mentions

- If you want a live USB secure OS, use [TailOS](https://tails.boum.org/).
  It will load to RAM and will leave no trace in the host computer.
  It also routes everything through Tor.
  There's a guide on how to use it in [anonymousplanet.org](https://anonymousplanet.org/guide.html#the-tails-route).
- [OpenBSD](https://www.openbsd.org/) is also a good alternative,
  but is is still very restrictive.
  It is a **solid option for servers**.
  Check [here for a lot of tutorials for daily usage](https://astro-gr.org/openbsd/).
- If you need an anonymous non-KYC eSIM card, use [silent.link](https://silent.link/).
- If you need a SMS activation code,
  use [sms4sats](https://sms4sats.com/),
  [juicysms](https://juicysms.com/),
  or [LNSMS](https://lnvpn.net/phone-numbers)

## License

This post is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International][cc-by-nc-sa].

[![CC BY-NC-SA 4.0][cc-by-nc-sa-image]][cc-by-nc-sa]

[cc-by-nc-sa]: http://creativecommons.org/licenses/by-nc-sa/4.0/
[cc-by-nc-sa-image]: https://licensebuttons.net/l/by-nc-sa/4.0/88x31.png
