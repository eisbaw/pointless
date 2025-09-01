Name:           pointless
Version:        0.1.0
Release:        1%{?dist}
Summary:        Keyboard-driven X11 cursor positioning utility

License:        MIT OR Apache-2.0
URL:            https://github.com/user/pointless
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  libX11-devel
BuildRequires:  libXext-devel
BuildRequires:  libXi-devel
BuildRequires:  pkg-config

# For vision grids (optional)
%if 0%{?fedora} || 0%{?rhel} >= 8
BuildRequires:  opencv-devel
BuildRequires:  clang-devel
%endif

Requires:       libX11%{?_isa}
Requires:       libXext%{?_isa}
Requires:       libXi%{?_isa}

%description
Pointless is a keyboard-driven cursor positioning utility for X11 that allows
precise cursor control using grid-based navigation. It provides multiple grid
types including homogeneous grids, QWERTY layout projection, arrow key
navigation, and Vim-style movement.

Features include:
* Multiple grid layouts (homogeneous, QWERTY, arrow, Vim)
* Configurable modifier keys and visual appearance
* High precision cursor positioning
* Optional computer vision-based grids (with OpenCV)
* Minimal resource usage and fast response times

%package vision
Summary:        Pointless with computer vision grid support
Requires:       %{name} = %{version}-%{release}
%if 0%{?fedora} || 0%{?rhel} >= 8
Requires:       opencv%{?_isa}
%endif

%description vision
This package provides Pointless with additional computer vision-based
grid types that can analyze screen content to provide intelligent
cursor positioning targets.

Additional vision grid types include:
* Hough line detection for technical drawings
* Rectangle detection for UI elements
* Edge density analysis for complex layouts
* Color clustering for design work
* Feature detection for detailed graphics

%prep
%autosetup

%build
# Set up Rust environment
export RUSTFLAGS="$RUSTFLAGS -C debuginfo=2"

# Build basic version
cargo build --release

# Build vision version if OpenCV is available
%if 0%{?fedora} || 0%{?rhel} >= 8
if pkg-config --exists opencv4; then
    cargo build --release --features vision-grids
fi
%endif

%install
# Install basic binary
install -D -m755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install vision binary if available
%if 0%{?fedora} || 0%{?rhel} >= 8
if [ -f target/release/%{name} ]; then
    # For now, vision grids are compiled into the same binary with feature flag
    # In the future, this could be a separate binary
    :
fi
%endif

# Install man page
install -D -m644 man/man1/%{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1

# Install documentation
install -D -m644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -D -m644 docs/USAGE.md %{buildroot}%{_docdir}/%{name}/USAGE.md
install -D -m644 docs/TUTORIAL.md %{buildroot}%{_docdir}/%{name}/TUTORIAL.md

# Install example scripts
install -d %{buildroot}%{_docdir}/%{name}/examples
install -m644 examples/*.sh %{buildroot}%{_docdir}/%{name}/examples/

%check
# Run unit tests (integration tests require X11)
cargo test --lib

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_mandir}/man1/%{name}.1*
%{_docdir}/%{name}/

%files vision
%doc docs/USAGE.md
%{_docdir}/%{name}/examples/

%changelog
* Mon Sep 01 2025 Pointless Maintainer <maintainer@example.com> - 0.1.0-1
- Initial RPM release
- Support for multiple grid types
- Optional computer vision features
- Comprehensive documentation and examples