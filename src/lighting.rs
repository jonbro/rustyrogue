/*
fn calc_normal(x : f32, y : f32) -> Vector3<f32> {
    let p = Vector3::new(x,y,0.0);
    const EPS : f32 = 1.0; // or some other value
    let h = Vector3::new(EPS,0.0, 0.0);
    return Vector3::new( get_height_v(p-h.xyy()) - get_height_v(p+h.xyy()),
                            get_height_v(p-h.yxy()) - get_height_v(p+h.yxy()),
                            2.0*h.x).normalize()
} 


fn get_height(x: f32, y: f32) -> f32 {
    const PI2 : f32 = 6.28;
    //(x/50.0*PI2*0.5).sin()*50.0
    //(x/80.0*PI2*0.5).sin()*40.0
   //((x/20.0*PI2*0.5).sin()+(y/12.5*PI2*0.5).sin())*50.0
    //(((x*0.18).sin()*(x*0.2).sin() + (y*0.23+x*0.15).cos())*0.5+1.0)*40.0
    // simple bumpy terrain
    // eventually would want to interpolate values from terrain heightmap
    ((x/40.0*PI2*0.5).sin()+(y/25.0*PI2*0.5).sin())*20.0
}
fn get_height_v(v : Vector3<f32>) -> f32{
    get_height(v.x, v.y)
}
fn blocked_v(v : Vector3<f32>) -> bool {
    const cx : i32 = 20;
    const cy : i32 = 20;
    // get the height of the terrain at the center, then consider any blocks within 10 of that blocked
    let h = get_height(cx as f32, cy as f32);
    false
}
*/