use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
    Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, GetDC, HBRUSH, HDC, SRCCOPY,
        StretchDIBits,
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        AdjustWindowRectEx, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, HCURSOR, HICON, IDC_ARROW, LoadCursorW, MSG, PEEK_MESSAGE_REMOVE_TYPE,
        PM_REMOVE, PeekMessageW, PostQuitMessage, RegisterClassW, SHOW_WINDOW_CMD, SW_SHOW,
        ShowWindow, TranslateMessage, WINDOW_EX_STYLE, WM_CLOSE, WM_DESTROY, WM_QUIT,
        WNDCLASS_STYLES, WNDCLASSW as WindowClass, WNDPROC, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

use renderer::*;

use windows::core::{PCWSTR, Result, w};

fn main() -> Result<()> {
    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CLOSE => {
                unsafe {
                    let _ = DestroyWindow(hwnd);
                }

                LRESULT(0)
            }

            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                }

                LRESULT(0)
            }

            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }
    //define window type
    let hInstance = unsafe { GetModuleHandleW(None)?.into() };

    let class = WindowClass {
        style: WNDCLASS_STYLES(0),
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance,
        hIcon: HICON::default(),
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        hbrBackground: HBRUSH::default(),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: w!("Renderer69"),
    };

    //register window class
    let atom = unsafe { RegisterClassW(&class) };
    if atom == 0 {
        return Err(windows::core::Error::from_thread());
    }

    //create window
    let dw_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
    let (buffer_w, buffer_h) = (1440, 640);
    let (nwidth, nheight) = get_window_size(buffer_w, buffer_h)?;
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("Renderer69"),
            w!("RENDERER 69"),
            dw_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            nwidth,
            nheight,
            None,
            None,
            Some(hInstance),
            None,
        )?
    };
    dbg!(hwnd);

    unsafe {
        ShowWindow(hwnd, SW_SHOW);
    }

    let mut screen = ScreenBuffer::new(buffer_w, buffer_h, Some(Color::Black as u32));
    //message loop
    let mut msg = MSG::default();

    //
    let bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: core::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: buffer_w as i32,
            biHeight: -(buffer_h as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };

    //acquire the window’s drawing context
    let hdc = unsafe { GetDC(Some(hwnd)) };
    if hdc.is_invalid() {
        return Err(windows::core::Error::from_thread());
    }

    present(&screen, &hdc, &bitmap_info);
    /*
    pub unsafe fn StretchDIBits(
        hdc: HDC,
        xdest: i32,
        ydest: i32,
        destwidth: i32,
        destheight: i32,
        xsrc: i32,
        ysrc: i32,
        srcwidth: i32,
        srcheight: i32,
        lpbits: Option<*const c_void>,
        lpbmi: *const BITMAPINFO,
        iusage: DIB_USAGE,
        rop: ROP_CODE,
    ) -> i32
        */

    let mut p1 = Vec3::new(0.0, 0.0, 4.0);
    let mut p2 = Vec3::new(1000.0, 500.0, 10.0);
    let mut p3 = Vec3::new(100.0, 700.0,10.0);
    let tri = Triangle3::new(p3, p2, p1);

    dbg!(tri.signed_area_twice());

    let fill = TriangleFillType::Gradient {
        c0: Color::Yellow.to_rgb(),
        c1: Color::White.to_rgb(),
        c2: Color::Black.to_rgb(),
    };

    let fill2 = TriangleFillType::Gradient {
        c0: Color::Red.to_rgb(),
        c1: Color::White.to_rgb(),
        c2: Color::Green.to_rgb(),
    };

    let mut x = 0.0;
    let mut pp1 = Vec3::new(x, 0.0, 5.0);
    let mut pp2 = Vec3::new(100.0, 200.0, 5.0);
    let mut pp3 = Vec3::new(800.0, 100.0, 5.0);
    let tri2 = Triangle3::new(pp3, pp2, pp1);

    let mut screen_triangles = vec![tri];

    for t in &screen_triangles{
        screen.fill_triangle_3d(*t, fill);
    }

    screen.clear(None);

    loop {
        while unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) }.as_bool() {
            if msg.message == WM_QUIT {
                return Ok(());
            }

            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        screen.clear(None);
        for mut t in &mut screen_triangles{
            t.p0.x += 3.0;
            t.p1.x += 3.0;
            t.p2.x += 3.0;
            screen.fill_triangle_3d(*t, fill);
        }

        present(&screen, &hdc, &bitmap_info);
    }

    Ok(())
}

fn get_window_size(buffer_w: u32, buffer_h: u32) -> Result<(i32, i32)> {
    let dw_ex_style = WINDOW_EX_STYLE(0);
    let dw_style = WS_OVERLAPPEDWINDOW;

    let mut window_rect = RECT {
        left: 0,
        top: 0,
        right: buffer_w as i32,
        bottom: buffer_h as i32,
    };

    unsafe {
        AdjustWindowRectEx(&mut window_rect, dw_style, false, dw_ex_style)?;
    }

    let window_w = window_rect.right - window_rect.left;
    let window_h = window_rect.bottom - window_rect.top;

    Ok((window_w, window_h))
}

fn present(screen: &ScreenBuffer, hdc: &HDC, bitmap_info: &BITMAPINFO) {
    let copied_lines = unsafe {
        StretchDIBits(
            *hdc,
            0,
            0,
            screen.w as i32,
            screen.h as i32,
            0,
            0,
            screen.w as i32,
            screen.h as i32,
            Some(screen.pixels().as_ptr().cast()),
            bitmap_info,
            DIB_RGB_COLORS,
            SRCCOPY,
        )
    };
    debug_assert!(copied_lines != 0);
}
