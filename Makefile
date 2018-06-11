#*******************************************************************************
#   Ledger App
#   (c) 2017 Ledger
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#*******************************************************************************

ifeq ($(BOLOS_SDK),)
$(error Environment variable BOLOS_SDK is not set)
endif
include $(BOLOS_SDK)/Makefile.defines

APPNAME ="Rusty"
APP_LOAD_PARAMS=--appFlags 0x50 --path "123'/435'" --curve ed25519 $(COMMON_LOAD_PARAMS)

APPVERSION_M=0
APPVERSION_N=1
APPVERSION_P=0
APPVERSION=$(APPVERSION_M).$(APPVERSION_N).$(APPVERSION_P)

ICONNAME=nanos_icon.gif

ifeq (customCA.key,$(wildcard customCA.key))
	CUSTOM_CA_PARAM=--rootPrivateKey `cat customCA.key`
else
	CUSTOM_CA_PARAM=
endif

################
# Default rule #
################

all: default

############
# Platform #
############

DEFINES   += OS_IO_SEPROXYHAL IO_SEPROXYHAL_BUFFER_SIZE_B=300
DEFINES   += HAVE_BAGL HAVE_SPRINTF
#DEFINES   += HAVE_PRINTF PRINTF=screen_printf
DEFINES   += PRINTF\(...\)=
DEFINES   += HAVE_IO_USB HAVE_L4_USBLIB IO_USB_MAX_ENDPOINTS=6 IO_HID_EP_LENGTH=64 HAVE_USB_APDU
DEFINES   += APP_MAJOR_VERSION=$(APPVERSION_M) APP_MINOR_VERSION=$(APPVERSION_N) APP_PATCH_VERSION=$(APPVERSION_P)

##############
# Compiler #
##############
#GCCPATH   := $(BOLOS_ENV)/gcc-arm-none-eabi-5_3-2016q1/bin/
#CLANGPATH := $(BOLOS_ENV)/clang-arm-fropi/bin/
CC       := $(CLANGPATH)clang

#CFLAGS   += -O0
CFLAGS   += -O3 -Os -Wno-typedef-redefinition

AS     := $(GCCPATH)arm-none-eabi-gcc

LD       := $(GCCPATH)arm-none-eabi-gcc
LDFLAGS  += -O3 -Os
LDLIBS   += -lm -lgcc -lc -z muldefs rusty-rs/target/thumbv6m-none-eabi/release/librusty_rs.a

# import rules to compile glyphs(/pone)
include $(BOLOS_SDK)/Makefile.glyphs

### variables processed by the common makefile.rules of the SDK to grab source files and include dirs
APP_SOURCE_PATH  += src
SDK_SOURCE_PATH  += lib_stusb
SDK_SOURCE_PATH  += lib_stusb_impl

load: all
	python -m ledgerblue.loadApp $(CUSTOM_CA_PARAM) $(APP_LOAD_PARAMS)

delete:
	python -m ledgerblue.deleteApp $(CUSTOM_CA_PARAM) $(COMMON_DELETE_PARAMS)

# import generic rules from the sdk
include $(BOLOS_SDK)/Makefile.rules

#add dependency on custom makefile filename
dep/%.d: %.c Makefile

