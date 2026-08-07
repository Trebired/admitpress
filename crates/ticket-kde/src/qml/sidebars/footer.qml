import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"
import "."

ColumnLayout {
  id: sidebarFooter

  property var release: null

  readonly property string organization: release ? release.organization || "" : ""
  readonly property string currentVersion: release ? release.current_version || "" : ""
  readonly property bool updateAvailable: release ? release.update_available === true : false
  readonly property string organizationUrl: release ? release.organization_url || "" : ""
  readonly property string appUrl: release ? release.app_url || "" : ""
  readonly property string latestVersion: release ? release.latest_version || "" : ""

  function domainOf(url) {
    const value = String(url || "")
    const marker = value.indexOf("://")
    const host = marker < 0 ? value : value.slice(marker + 3)
    const slash = host.indexOf("/")
    return slash < 0 ? host : host.slice(0, slash)
  }

  spacing: Kirigami.Units.smallSpacing

  Kirigami.Separator {
    Layout.fillWidth: true
  }

  Controls.Label {
    Layout.fillWidth: true
    Layout.leftMargin: Kirigami.Units.largeSpacing
    Layout.rightMargin: Kirigami.Units.largeSpacing
    visible: sidebarFooter.updateAvailable
    wrapMode: Text.WordWrap
    color: Kirigami.Theme.positiveTextColor
    font.pointSize: Kirigami.Theme.smallFont.pointSize
    text: LocaleText.tr("Version ") + sidebarFooter.latestVersion + LocaleText.tr(" is available")
  }

  RowLayout {
    Layout.fillWidth: true
    Layout.leftMargin: Kirigami.Units.largeSpacing
    Layout.rightMargin: Kirigami.Units.largeSpacing
    spacing: Kirigami.Units.largeSpacing

    SidebarLink {
      url: sidebarFooter.organizationUrl
      text: sidebarFooter.domainOf(sidebarFooter.organizationUrl)
    }

    SidebarLink {
      url: sidebarFooter.appUrl
      text: sidebarFooter.domainOf(sidebarFooter.appUrl)
    }

    Item {
      Layout.fillWidth: true
    }
  }

  Controls.Label {
    Layout.fillWidth: true
    Layout.leftMargin: Kirigami.Units.largeSpacing
    Layout.rightMargin: Kirigami.Units.largeSpacing
    Layout.bottomMargin: Kirigami.Units.largeSpacing
    wrapMode: Text.WordWrap
    color: Kirigami.Theme.disabledTextColor
    font.pointSize: Kirigami.Theme.smallFont.pointSize
    text: LocaleText.tr("Made by ") + sidebarFooter.organization
    + "\n" + LocaleText.tr("Version ") + sidebarFooter.currentVersion
  }
}
