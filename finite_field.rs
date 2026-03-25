//This is a vibe coded implementaion of finite field concepts and rules in Rust
//CREATED WITH GPT'S CODEX TOOL
use std::fmt;

// We'll use GF(5) as the simplest example of a prime finite field.
const PRIME: u32 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fp<const P: u32> {
    // Every value is always stored in the range 0..P-1.
    value: u32,
}

impl<const P: u32> Fp<P> {
    fn new(value: i32) -> Self {
        // rem_euclid keeps negative inputs valid too, so -1 becomes P - 1.
        let p = P as i32;
        let reduced = value.rem_euclid(p) as u32;
        Self { value: reduced }
    }

    fn zero() -> Self {
        Self { value: 0 }
    }

    fn one() -> Self {
        Self { value: 1 % P }
    }

    fn add(self, other: Self) -> Self {
        Self::new(self.value as i32 + other.value as i32)
    }

    fn mul(self, other: Self) -> Self {
        Self::new((self.value * other.value) as i32)
    }

    fn neg(self) -> Self {
        Self::new(-(self.value as i32))
    }

    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }

    fn pow(self, exp: u32) -> Self {
        // Fast exponentiation lets us compute powers efficiently.
        let mut result = Self::one();
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                result = result.mul(base);
            }
            base = base.mul(base);
            e /= 2;
        }
        result
    }

    fn inverse(self) -> Option<Self> {
        if self == Self::zero() {
            None
        } else {
            // In GF(p), a^(p-2) is the multiplicative inverse of a when a != 0.
            Some(self.pow(P - 2))
        }
    }
}

impl<const P: u32> fmt::Display for Fp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GF8 {
    // Represents a0 + a1*x + a2*x^2 over GF(2).
    a0: u8,
    a1: u8,
    a2: u8,
}

impl GF8 {
    fn new(a0: u8, a1: u8, a2: u8) -> Self {
        Self {
            a0: a0 % 2,
            a1: a1 % 2,
            a2: a2 % 2,
        }
    }

    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn one() -> Self {
        Self::new(1, 0, 0)
    }

    fn alpha() -> Self {
        // "alpha" is the residue class of x.
        Self::new(0, 1, 0)
    }

    fn elements() -> [Self; 8] {
        // There are exactly 2^3 = 8 possible coefficient triples.
        [
            Self::new(0, 0, 0),
            Self::new(1, 0, 0),
            Self::new(0, 1, 0),
            Self::new(0, 0, 1),
            Self::new(1, 1, 0),
            Self::new(1, 0, 1),
            Self::new(0, 1, 1),
            Self::new(1, 1, 1),
        ]
    }

    fn add(self, other: Self) -> Self {
        // In GF(2), addition is XOR on coefficients.
        Self::new(self.a0 ^ other.a0, self.a1 ^ other.a1, self.a2 ^ other.a2)
    }

    fn mul(self, other: Self) -> Self {
        // First multiply as polynomials before reducing.
        let a = [self.a0, self.a1, self.a2];
        let b = [other.a0, other.a1, other.a2];
        let mut raw = [0u8; 5];

        for i in 0..3 {
            for j in 0..3 {
                raw[i + j] ^= a[i] & b[j];
            }
        }

        // Reduce modulo x^3 + x + 1, so x^3 = x + 1 and x^4 = x^2 + x.
        let c0 = raw[0] ^ raw[3];
        let c1 = raw[1] ^ raw[3] ^ raw[4];
        let c2 = raw[2] ^ raw[4];
        Self::new(c0, c1, c2)
    }

    fn square(self) -> Self {
        self.mul(self)
    }
}

impl fmt::Display for GF8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.a2 == 1 {
            parts.push("a^2");
        }
        if self.a1 == 1 {
            parts.push("a");
        }
        if self.a0 == 1 || parts.is_empty() {
            parts.push("1");
        }
        write!(f, "{}", parts.join(" + "))
    }
}

fn assert_and_print(label: &str, ok: bool) {
    let status = if ok { "true" } else { "false" };
    println!("{label:<34} {status}");
}

fn verify_prime_field_rules() {
    // This alias makes the rest of the demo read like math notation.
    type F = Fp<PRIME>;

    let a = F::new(2);
    let b = F::new(4);
    let c = F::new(3);

    println!("Finite field example: GF({PRIME})");
    println!("Elements are {{0, 1, 2, 3, 4}} with arithmetic mod {PRIME}.");
    println!();

    // Each check corresponds to one of the usual field rules.
    assert_and_print("Addition closure:", a.add(b) == F::new(6));
    println!("2 + 4 = {} (mod {PRIME})", a.add(b));

    assert_and_print("Multiplication closure:", b.mul(c) == F::new(12));
    println!("4 * 3 = {} (mod {PRIME})", b.mul(c));

    assert_and_print("Additive identity:", a.add(F::zero()) == a);
    assert_and_print("Multiplicative identity:", a.mul(F::one()) == a);
    assert_and_print("Additive inverse:", a.add(a.neg()) == F::zero());
    assert_and_print("Commutative addition:", a.add(b) == b.add(a));
    assert_and_print("Commutative multiplication:", a.mul(b) == b.mul(a));
    assert_and_print("Associative addition:", a.add(b).add(c) == a.add(b.add(c)));
    assert_and_print("Associative multiplication:", a.mul(b).mul(c) == a.mul(b.mul(c)));
    assert_and_print(
        "Distributive law:",
        a.mul(b.add(c)) == a.mul(b).add(a.mul(c)),
    );

    let inverse = b.inverse().expect("non-zero element has an inverse");
    assert_and_print("Multiplicative inverse exists:", b.mul(inverse) == F::one());
    println!("Inverse of 4 is {} because 4 * {} = 1.", inverse, inverse);
    println!("2 - 4 = {} (mod {PRIME})", a.sub(b));
    println!();
}

fn show_extension_field_example() {
    // This second example shows that not every finite field is just integers mod p.
    let alpha = GF8::alpha();
    let one = GF8::one();
    let elements = GF8::elements();

    println!("Extension field example: GF(2^3)");
    println!("We build it as GF(2)[x] / (x^3 + x + 1). Let a be the class of x.");
    println!("Then a^3 = a + 1 inside the field.");
    println!();

    let alpha_squared = alpha.square();
    let alpha_cubed = alpha_squared.mul(alpha);
    let alpha_plus_one = alpha.add(one);
    assert_and_print("Reduction rule a^3 = a + 1:", alpha_cubed == alpha_plus_one);
    println!("a^2 = {}", alpha_squared);
    println!("a^3 = {}", alpha_cubed);

    // We verify distributivity with a concrete symbolic example.
    let left = alpha.mul(alpha.add(one));
    let right = alpha.mul(alpha).add(alpha.mul(one));
    assert_and_print("Distributive law in GF(2^3):", left == right);
    println!("a(a + 1) = {}", left);

    let closure_example = GF8::new(1, 1, 0).mul(GF8::new(0, 1, 1));
    // Closure means multiplying any two field elements stays inside the field.
    let closure_holds = elements
        .iter()
        .copied()
        .all(|x| elements.iter().copied().all(|y| elements.contains(&x.mul(y))));
    assert_and_print("Closure in GF(2^3):", closure_holds);
    println!("(1 + a)(a + a^2) = {}", closure_example);
}

fn main() {
    // Run both demos so the output shows a prime field and an extension field.
    println!("Finite field rules demo");
    println!("=======================");
    println!();
    verify_prime_field_rules();
    show_extension_field_example();
}
