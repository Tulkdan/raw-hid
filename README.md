# Raw Hid experimentation

This project is an experimentation to integrate with my keyboard that has [QMK via raw hid](https://beta.docs.qmk.fm/using-qmk/software-features/feature_rawhid)

## Dependencies

The rust lib needs some dependencies to work properly as of it is an abstraction of C code. If you're using Ubuntu you need to have both installed to recognize the keyboard and be able to send bytes to it.

* libudev-dev
* libusb-1.0-0-dev

#### Explain to me like I'm 5

Now I can send _beep_ to my keyboard and he you say _beep boop beep boop_