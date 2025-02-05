%global _cross_first_party 1
%undefine _debugsource_packages

Name: %{_cross_os}nvidia-migmanager
Version: 0.1
Release: 1%{?dist}
Epoch: 1
Summary: Tool to generate NVIDIA MIG Binary and config files
License: Apache-2.0 OR MIT
URL: https://github.com/bottlerocket-os/bottlerocket

Source100: nvidia-migmanager.service
Source101: nvidia-migmanager-tmpfiles.conf
Source102: mig-reboot-if-required.service.drop-in.conf

%description
%{summary}.

%prep
%setup -T -c
%cargo_prep

%build
%cargo_build --manifest-path %{_builddir}/sources/Cargo.toml -p nvidia-migmanager

%install
install -d %{buildroot}%{_cross_bindir}
install -p -m 0755 %{__cargo_outdir}/nvidia-migmanager %{buildroot}%{_cross_bindir}

install -d %{buildroot}%{_cross_unitdir}
install -p -m 0644 %{S:100} %{buildroot}%{_cross_unitdir}

install -d %{buildroot}%{_cross_tmpfilesdir}
install -p -m 0644 %{S:101} %{buildroot}%{_cross_tmpfilesdir}/nvidia-migmanager.conf

install -d %{buildroot}%{_cross_unitdir}/reboot-if-required.service.d
install -p -m 0644 %{S:102} %{buildroot}%{_cross_unitdir}/reboot-if-required.service.d/mig-gpu-reset.conf

%files
%{_cross_bindir}/nvidia-migmanager
%{_cross_unitdir}/nvidia-migmanager.service
%{_cross_tmpfilesdir}/nvidia-migmanager.conf
%{_cross_unitdir}/reboot-if-required.service.d/mig-gpu-reset.conf
