# Maintainer: TostetLolz <me@toster.lol>
<<<<<<< HEAD
pkgname=grabbie-git
pkgver=latest
pkgrel=1
pkgdesc="Grabbie is a tool to download files and resources from various sources (latest, from git)."
arch=('x86_64')
url="https://github.com/tosterlolz/grabbie"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo' 'git')
provides=('grabbie')
conflicts=('grabbie')
source=("$pkgname::git+$url.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --long --tags 2>/dev/null || echo "r$(git rev-list --count HEAD).$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/$pkgname"
=======
pkgname=grabbie
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
  cd "$pkgname-$pkgver"
>>>>>>> 071f15081362d9d6c16a44240dde7e2b1a73050e
  cargo build --release --locked
}

package() {
<<<<<<< HEAD
  cd "$srcdir/$pkgname"
  install -Dm755 "target/release/grabbie" "$pkgdir/usr/bin/grabbie"
  install -Dm644 README.md "$pkgdir/usr/share/doc/grabbie/README.md"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/grabbie/LICENSE"
}
=======
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/grabbie" "$pkgdir/usr/bin/grabbie"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
>>>>>>> 071f15081362d9d6c16a44240dde7e2b1a73050e
