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

    use Color as C;
    let mut p1 = Vec3::new(0.0, 0.0, 10.0);
    let mut p2 = Vec3::new(1000.0, 300.0, -70.0);
    let mut p3 = Vec3::new(100.0, 300.0, -100.0);
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

    let mut x = 600.0;
    let mut pp1 = Vec3::new(200.0, 200.0, -120.0);
    let mut pp2 = Vec3::new(500.0, 500.0, 0.0);
    let mut pp3 = Vec3::new(x, 300.0, -100.0);
    let tri2 = Triangle3::new(pp1, pp2, pp3);

    let mut center_z = 0.0;
    let mut center_x = 1000.0;
    let mut center = Vec3::new(center_x, 300.0, center_z);
    let sphere = Mesh::uv_sphere(center, 100.0, 90, 120);
    screen.render_mesh(&sphere, Color::Yellow as u32);
    screen.render_mesh(&tri2.into(), Color::Red as u32);
    screen.render_mesh(&tri.into(), Color::Blue as u32);

    screen.draw_triangle_3d(tri2, Color::Green as u32);
    let torus = Mesh::torus(Vec3::new(800.0, 400.0, -150.0), 250.0, 80.0, 48, 24, -0.79);
    let mut screen_meshes = vec![
        (torus, Color::Magenta),
        (tri.into(), C::Red),
        (tri2.into(), C::Green),
        (sphere, C::Yellow),
    ];

    screen.draw_line_depth(Vec3::new(800.0, 300.0, -0.0), center, 0);
    //screen.draw_line(center.into(), Vec3::new(800.0, 300.0, 0.0).into(), 0);

    let mut left = true;
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

        for (m, c) in screen_meshes.iter() {
            screen.render_mesh(m, *c as u32);
        }

        if center_x <= -1.0 {
            left = false;
        } else if center_x > 1000.0 {
            left = true;
        }

        if left {
            center_x -= 5.0;
        } else {
            center_x += 10.0;
        }

        let mut center = Vec3::new(center_x, 250.0, center_z);
        let sphere = Mesh::uv_sphere(center, 100.0, 90, 120);

        screen_meshes.last_mut().unwrap().0 = sphere;

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
