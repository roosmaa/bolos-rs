/*******************************************************************************
*   $NANO Wallet for Ledger Nano S & Blue
*   (c) 2018 Mart Roosmaa
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/

#include <string.h>
#include <stdbool.h>

#include "os.h"
#include "os_io_seproxyhal.h"

#include "glyphs.h"

#if defined(TARGET_NANOS)

ux_state_t ux;

char textLabel[50];

void ui_ticker_event(bool uxAllowed) {
}

/***
 * Display address
 */

const bagl_element_t ui_display_address[] = {
    {{/* type */ BAGL_RECTANGLE, /* userid */ 0x00,
      /* x */ 0, /* y */ 0, /* width */ 128, /* height */ 32,
      /* stroke */ 0, /* radius */ 0, /* fill */ BAGL_FILL,
      /* fgcolor */ 0x000000, /* bgcolor */ 0xFFFFFF,
      /* font_id */ 0, /* icon_id */ 0},
     /* text */ NULL, /* touch_area_brim */ 0,
     /* overfgcolor */ 0, /* overbgcolor */ 0,
     /* tap */ NULL, /* out */ NULL, /* over */ NULL},

    {{/* type */ BAGL_ICON, /* userid */ 0x00,
      /* x */ 3, /* y */ 12, /* width */ 7, /* height */ 7,
      /* stroke */ 0, /* radius */ 0, /* fill */ 0,
      /* fgcolor */ 0xFFFFFF, /* bgcolor */ 0x000000,
      /* font_id */ 0, /* icon_id */ BAGL_GLYPH_ICON_CROSS},
     /* text */ NULL, /* touch_area_brim */ 0,
     /* overfgcolor */ 0, /* overbgcolor */ 0,
     /* tap */ NULL, /* out */ NULL, /* over */ NULL},

    {{/* type */ BAGL_ICON, /* userid */ 0x00,
      /* x */ 117, /* y */ 13, /* width */ 8, /* height */ 6,
      /* stroke */ 0, /* radius */ 0, /* fill */ 0,
      /* fgcolor */ 0xFFFFFF, /* bgcolor */ 0x000000,
      /* font_id */ 0, /* icon_id */ BAGL_GLYPH_ICON_CHECK},
     /* text */ NULL, /* touch_area_brim */ 0,
     /* overfgcolor */ 0, /* overbgcolor */ 0,
     /* tap */ NULL, /* out */ NULL, /* over */ NULL},

    {{/* type */ BAGL_LABELINE, /* userid */ 0x00,
      /* x */ 0, /* y */ 12, /* width */ 128, /* height */ 12,
      /* scrolldelay */ 0, /* radius */ 0, /* fill */ 0,
      /* fgcolor */ 0xFFFFFF, /* bgcolor */ 0x000000,
      /* font_id */ BAGL_FONT_OPEN_SANS_REGULAR_11px | BAGL_FONT_ALIGNMENT_CENTER,
      /* scrollspeed */ 0},
     /* text */ "Hello!", /* touch_area_brim */ 0,
     /* overfgcolor */ 0, /* overbgcolor */ 0,
     /* tap */ NULL, /* out */ NULL, /* over */ NULL},
    {{/* type */ BAGL_LABELINE, /* userid */ 0x02,
      /* x */ 23, /* y */ 26, /* width */ 82, /* height */ 12,
      /* scrolldelay */ 10 | BAGL_STROKE_FLAG_ONESHOT,
      /* radius */ 0, /* fill */ 0,
      /* fgcolor */ 0xFFFFFF, /* bgcolor */ 0x000000,
      /* font_id */ BAGL_FONT_OPEN_SANS_EXTRABOLD_11px | BAGL_FONT_ALIGNMENT_CENTER,
      /* scrollspeed */ 26},
     /* text */ textLabel, /* touch_area_brim */ 0,
     /* overfgcolor */ 0, /* overbgcolor */ 0,
     /* tap */ NULL, /* out */ NULL, /* over */ NULL},
};

const bagl_element_t *ui_display_address_prepro(const bagl_element_t *element) {
    if (element->component.userid == 2) {
        UX_CALLBACK_SET_INTERVAL(MAX(
            3000, 1000 + bagl_label_roundtrip_duration_ms(element, 7)));
    }
    return element;
}

uint32_t ui_display_address_button(uint32_t button_mask,
                                   uint32_t button_mask_counter) {
    switch (button_mask) {
    case BUTTON_EVT_RELEASED | BUTTON_LEFT:
        os_sched_exit(0);
        return 0;

    case BUTTON_EVT_RELEASED | BUTTON_RIGHT:
        return 0;

    // For other button combinations return early and do nothing
    default:
        return 0;
    }
}

void ui_idle(void) {
    strcpy(textLabel, "WORLD!");
    UX_DISPLAY(ui_display_address, ui_display_address_prepro);
}

#endif // defined(TARGET_NANOS)
