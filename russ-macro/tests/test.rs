use russ::static_css;

#[test]
fn test() {
    let test = {
        struct A {
            b: String,
        }

        A { b: "test".into() }
    };

    assert_eq!(test.b, "test".to_string());

    let styles = static_css! {
        wrapper {
            display: flex;
        }

        content {
            font-size: 2rem;
            text-align: center;
        }
    };
}
