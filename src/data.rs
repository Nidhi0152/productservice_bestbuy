use crate::configuration::Settings;
use crate::model::Product;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
    Product {
        id: 1,
        name: "Sony WH-1000XM5 Wireless Noise-Cancelling Headphones".to_string(),
        price: 499.99,
        description: "Enjoy industry-leading noise cancellation and superior sound quality with Sony's WH-1000XM5 over-ear headphones. With up to 30 hours of battery life and multipoint connectivity.".to_string(),
        image: "/sony_wh1000xm5.jpg".to_string()
    },
    Product {
        id: 2,
        name: "Samsung 55\" 4K UHD HDR QLED Smart TV (Q80D)".to_string(),
        price: 1399.99,
        description: "Dive into cinematic entertainment with Samsung's Q80D QLED TV. Featuring Quantum HDR, Motion Xcelerator Turbo+, and built-in Alexa for smart control.".to_string(),
        image: "/samsung_qled_55.jpg".to_string()
    },
    Product {
        id: 3,
        name: "Logitech MX Master 3S Wireless Mouse".to_string(),
        price: 129.99,
        description: "Enhance productivity with the Logitech MX Master 3S. Features ultra-fast scrolling, 8000 DPI sensor, and ergonomic design for long hours of work.".to_string(),
        image: "/logitech_mx3s.jpg".to_string()
    },
    Product {
        id: 4,
        name: "Apple iPad Air 11\" (2024, M2 chip, Wi-Fi, 128GB)".to_string(),
        price: 849.99,
        description: "The new iPad Air with the M2 chip offers blazing performance in a sleek design. Perfect for creative professionals, students, and entertainment on the go.".to_string(),
        image: "/ipad_air_m2.jpg".to_string()
    },
    Product {
        id: 5,
        name: "Breville Barista Express Espresso Machine".to_string(),
        price: 899.99,
        description: "Brew café-style espresso at home with this all-in-one machine. Integrated grinder, steam wand, and customizable settings for perfect shots every time.".to_string(),
        image: "/breville_barista_express.jpg".to_string()
    },
    Product {
        id: 6,
        name: "Jabra Elite 8 Active True Wireless Earbuds".to_string(),
        price: 229.99,
        description: "Waterproof, dustproof, and shock-resistant earbuds with spatial sound, adaptive ANC, and up to 56 hours of battery with case.".to_string(),
        image: "/jabra_elite8.jpg".to_string()
    },
    Product {
        id: 7,
        name: "HP Envy x360 15.6\" Touch Laptop (Ryzen 7, 16GB RAM, 1TB SSD)".to_string(),
        price: 1149.99,
        description: "Versatile and powerful 2-in-1 laptop for work and play. AMD Ryzen 7 processor and large SSD make it perfect for multitasking.".to_string(),
        image: "/hp_envy_x360.jpg".to_string()
    },
    Product {
        id: 8,
        name: "Sony PlayStation 5 Console – Disc Edition".to_string(),
        price: 649.99,
        description: "Experience next-gen gaming with stunning visuals and ultra-fast load times on the PS5 console. Includes DualSense controller.".to_string(),
        image: "/ps5_disc.jpg".to_string()
    },
    Product {
        id: 9,
        name: "Instant Pot Duo Crisp + Air Fryer 8-Quart".to_string(),
        price: 229.99,
        description: "Multi-functional cooker with pressure, slow cook, steam, sauté, air fry, bake, roast, and more. Cook healthy meals faster.".to_string(),
        image: "/instant_pot_duo_crisp.jpg".to_string()
    },
    Product {
        id: 10,
        name: "GoPro HERO12 Black Action Camera".to_string(),
        price: 539.99,
        description: "Capture adventures in 5.3K video and 27MP photos. Waterproof, HyperSmooth 6.0 stabilization, and time-lapse features.".to_string(),
        image: "/gopro_hero12.jpg".to_string()
    },
    Product {
        id: 11,
        name: "Dell UltraSharp 27\" 4K Monitor (U2723QE)".to_string(),
        price: 799.99,
        description: "Enjoy incredible clarity and detail with this 4K IPS monitor. Features USB-C hub and ComfortView Plus technology.".to_string(),
        image: "/dell_ultrasharp.jpg".to_string()
    },
    Product {
        id: 12,
        name: "Google Pixel 8 Pro 128GB (Unlocked)".to_string(),
        price: 1199.99,
        description: "Pixel 8 Pro combines AI photography, 120Hz OLED display, and the powerful Tensor G3 chip for fast, secure performance.".to_string(),
        image: "/pixel8pro.jpg".to_string()
    },
    Product {
        id: 13,
        name: "Yeti Rambler 26oz Stainless Steel Water Bottle".to_string(),
        price: 49.99,
        description: "Durable, double-wall vacuum insulated bottle to keep drinks cold or hot. Features leakproof chug cap.".to_string(),
        image: "/yeti_rambler.jpg".to_string()
    },
    Product {
        id: 14,
        name: "Kindle Paperwhite Signature Edition (32GB, 6.8” Display)".to_string(),
        price: 239.99,
        description: "Read comfortably with an adjustable warm light, wireless charging, and a glare-free display – even in direct sunlight.".to_string(),
        image: "/kindle_paperwhite.jpg".to_string()
    },
    Product {
        id: 15,
        name: "Anker 737 Power Bank (PowerCore 24K)".to_string(),
        price: 189.99,
        description: "Charge your devices fast with 140W USB-C output. 24,000mAh capacity ideal for phones, tablets, and laptops on the go.".to_string(),
        image: "/anker_737.jpg".to_string()
    },
    Product {
        id: 16,
        name: "Ring Video Doorbell Pro 2".to_string(),
        price: 329.99,
        description: "See who’s at your door in 1536p HD with head-to-toe video, 3D motion detection, and built-in Alexa greetings.".to_string(),
        image: "/ring_pro2.jpg".to_string()
    },
    Product {
        id: 17,
        name: "Samsung Galaxy Watch 6 Classic (47mm, LTE)".to_string(),
        price: 599.99,
        description: "Track your fitness, monitor heart rate, and answer calls with this stylish, rotating bezel smartwatch.".to_string(),
        image: "/galaxy_watch6.jpg".to_string()
    },
    Product {
        id: 18,
        name: "SteelSeries Apex Pro TKL Wireless Gaming Keyboard".to_string(),
        price: 349.99,
        description: "Next-gen gaming keyboard with OmniPoint 2.0 switches, per-key RGB, and adjustable actuation. Built for elite gamers.".to_string(),
        image: "/steelseries_apex.jpg".to_string()
    }
]
}
