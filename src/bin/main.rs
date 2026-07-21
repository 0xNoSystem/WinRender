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

    let mut screen = ScreenBuffer::new(buffer_w, buffer_h, Some(Color::Cyan as u32));
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

    let mut R = Renderer { screen };

    present(&R.screen, &hdc, &bitmap_info);
    let (mut mesh_store, mut material_store, mut obj_store) = init_scene();

    let (solid_yellow, solid_blue, full_red) = (
        material_store.insert(Material::new(Color::Yellow as u32, CullMode::Back)),
        material_store.insert(Material::new(Color::Blue as u32, CullMode::Back)),
        material_store.insert(Material::new_no_cull(Color::Red as u32)),
    );

    let sphere = Sphere {
        radius: 120.0,
        lat_steps: 32,
        long_steps: 48,
    };
    let sphere_mesh_id = mesh_store.insert(sphere.mesh());
    let sphere1_obj_id = obj_store.create_object(ObjectSpec {
        mesh_id: sphere_mesh_id,
        material_id: solid_yellow,
        transform: Transform3D::default(),
        visible: true,
    });

    let sphere2_obj_id = obj_store.create_object(ObjectSpec {
        mesh_id: sphere_mesh_id,
        material_id: solid_blue,
        transform: Transform3D{
            position: Vec3::new(600.0, 300.0, -150.0),
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        },
        visible: true,
    });

    let triangle = Triangle3 {
        p0: Vec3::new(-1.0, -1.0, 0.0),
        p1: Vec3::new(0.0, 1.0, 0.0),
        p2: Vec3::new(1.0, -1.0, 0.0),
    };

    let triangle_mesh_id = mesh_store.insert(Mesh::from(triangle));

    let t1 = obj_store.create_object(ObjectSpec {
        mesh_id: triangle_mesh_id,
        material_id: full_red,
        transform: Transform3D {
            position: Vec3::new(600.0, 300.0, -100.0),
            rotation: Vec3::ZERO,
            scale: Vec3::new(300.0, 200.0, 1.0),
        },
        visible: true,
    });

    present(&R.screen, &hdc, &bitmap_info);
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

        R.screen.clear(None);
        if let Some(sphere_object) = obj_store.get_mut(sphere1_obj_id) {
            sphere_object.transform.position.x += 4.0;
            sphere_object.transform.position.y += 2.0;
            sphere_object.transform.position.z -= 1.0;

            let position = sphere_object.transform.position;
            let scale = sphere_object.transform.scale;

            let radius_x = 120.0 * scale.x.abs();
            let radius_y = 120.0 * scale.y.abs();

            let fully_outside = position.x - radius_x >= R.screen.w as f32
                || position.y - radius_y >= R.screen.h as f32
                || position.x + radius_x < 0.0
                || position.y + radius_y < 0.0;

            if fully_outside {
                sphere_object.transform.position = Vec3::ZERO;
            }
        }

        if let Some(tri_object) = obj_store.get_mut(t1) {
            tri_object.transform.rotation.y += 0.02;
        }
        for object in obj_store.iter_mut() {
            if let Some(mesh) = mesh_store.get(object.mesh_id)
                && let Some(material) = material_store.get(object.material_id)
            {
                R.draw_mesh(object, mesh, material);
            }
        }
        present(&R.screen, &hdc, &bitmap_info);
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

fn init_scene() -> (MeshStore, MaterialStore, ObjectStore) {
    (MeshStore::new(), MaterialStore::new(), ObjectStore::new())
}
