pkgname=Grabbie
pkgver=0.1.0
pkgrel=1
pkgdesc="Grabbie is a tool to grab, download, or manage files and resources from various sources."
arch=('x86_64')
url="https://github.com/tosterlolz/Grabbie"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/grabbie" "$pkgdir/usr/bin/grabbie"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE" # Uncomment if LICENSE exists
}
