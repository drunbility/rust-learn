#[derive(Debug)]
struct Point(i32,i32);

fn cab_distance(p1:&Point,p2:&Point) -> i32 {
    (p1.0-p2.0).abs()+(p1.1-p2.1).abs()
}


fn find_nearest<'a,'q>(points:&'q [Point],query:&'a Point) -> Option<&'q Point> {
    let mut nearest = None;
    for p in points {
        if let Some((_,nearest_dist)) = nearest {
            let dist = cab_distance(p,query);
            if dist < nearest_dist {
                nearest = Some((p,dist));
            } 
        } else {
            nearest = Some((p,cab_distance(p,query)));
        }
    };

    nearest.map(|(p,_)|p)
}


fn main() {
    let points = &[Point(1,0),Point(1,0),Point(-1,0),Point(-1,0)];
    let nearest = {
        let query = Point(0,2);
        find_nearest(points,&query)
    };
    println!("{:?}",nearest);
}
