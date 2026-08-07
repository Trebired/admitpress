Name:           %{app_name}
Version:        %{app_version}
Release:        1%{?dist}
Summary:        Design and print sequential admission tickets

License:        AGPL-3.0-only
URL:            https://github.com/trebired/admitpress

Requires:       qt6-qtbase
Requires:       qt6-qtdeclarative
Requires:       kf6-kirigami
Requires:       kf6-qqc2-desktop-style
Requires:       cups-client
Requires:       librsvg2-tools
Requires:       poppler-utils
Requires:       polkit
Requires:       breeze-icon-theme
Recommends:     adwaita-icon-theme
Recommends:     xdg-desktop-portal-gtk

%description
This application designs admission tickets and prints them in order, so every ticket
that leaves the printer carries the next number in the sequence. It keeps a
daily counter for each price, records every sale, and submits print jobs to
CUPS.

%install
cp -a %{_stagedir}/. %{buildroot}/

%files
%license %{_datadir}/licenses/%{app_name}/LICENSE
%{_bindir}/%{app_name}
%{_datadir}/applications/app.ticket.native.desktop
%{_datadir}/icons/hicolor/scalable/apps/app.ticket.native.svg
%{_datadir}/metainfo/app.ticket.native.metainfo.xml

%changelog
