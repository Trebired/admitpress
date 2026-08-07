import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.kirigami.layouts as KirigamiLayouts
import "../components"

Kirigami.ScrollablePage {
  id: homePage
  required property var root
  required property var addTicketAction
  required property var ticketsAction
  required property var settingsAction
  title: LocaleText.tr("Tickets")
  actions: [ticketsAction, settingsAction]
  Component.onCompleted: PageColumn.configure(KirigamiLayouts.ColumnView, {
      minimumWidth: Kirigami.Units.gridUnit * 22,
      preferredWidth: Kirigami.Units.gridUnit * 26,
      interactiveResizeEnabled: true
  })

  ColumnLayout {
    width: homePage.availableWidth
    spacing: Kirigami.Units.largeSpacing

    Kirigami.InlineMessage {
      Layout.fillWidth: true
      visible: root.appState.development_mode
      type: Kirigami.MessageType.Information
      text: LocaleText.tr("Development output is enabled. Print actions create SVG files under ") +
        root.appState.data_dir +
        LocaleText.tr("/development-prints and do not send paper to a printer.")
      actions: [
        Kirigami.Action {
          text: LocaleText.tr("Settings")
          onTriggered: root.settingsDialogObject.open()
        }
      ]
    }

    Kirigami.PlaceholderMessage {
      Layout.fillWidth: true
      Layout.minimumHeight: 360
      visible: !root.appState.tickets || root.appState.tickets.length === 0
      text: LocaleText.tr("No tickets yet")
      explanation: LocaleText.tr("Create a ticket, position dynamic fields, add prices, and start printing.")
      helpfulAction: addTicketAction
    }

    Repeater {
      model: root.appState.tickets || []

      delegate: Kirigami.Card {
        required property var modelData

        Layout.fillWidth: true
        banner.title: modelData.ticket.name
        banner.source: root.fileUrl(modelData.ticket.background_svg_path)

        contentItem: ColumnLayout {
          spacing: Kirigami.Units.smallSpacing

          ColumnLayout {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            ColumnLayout {
              Layout.fillWidth: true
              Controls.Label {
                text: Number(modelData.ticket.width_mm / 10).toLocaleString(Qt.locale(), "f", 1)
                + " × "
                + Number(modelData.ticket.height_mm / 10).toLocaleString(Qt.locale(), "f", 1)
                + " cm"
                color: Kirigami.Theme.disabledTextColor
              }
              Controls.Label {
                text: modelData.subtickets.length + " " + LocaleText.tr("prices")
              }
              Controls.Label {
                Layout.fillWidth: true
                visible: modelData.subtickets.length > 0
                text: modelData.subtickets.length > 0
                ? LocaleText.tr("Next today: ") + root.ticketNextSummary(modelData)
                : ""
                elide: Text.ElideRight
              }
              Controls.Label {
                text: root.selectedPrinterQueue()
                ? LocaleText.tr("Printer: ") + root.selectedPrinterQueue()
                : LocaleText.tr("No physical printer selected")
                color: root.selectedPrinterQueue()
                ? Kirigami.Theme.textColor
                : Kirigami.Theme.neutralTextColor
              }
            }

            Flow {
              Layout.fillWidth: true
              spacing: Kirigami.Units.smallSpacing

              Controls.Button {
                text: LocaleText.tr("Edit")
                icon.name: AppIcons.pick("edit")
                onClicked: root.openEditor(modelData.ticket.id)
              }
              Controls.Button {
                text: LocaleText.tr("Start printing")
                icon.name: AppIcons.pick("printer")
                highlighted: true
                onClicked: root.openPrint(modelData.ticket.id)
              }
              Controls.ToolButton {
                text: LocaleText.tr("More")
                icon.name: AppIcons.pick("overflow")
                onClicked: ticketMenu.popup()

                Controls.Menu {
                  id: ticketMenu
                  DangerMenuItem {
                    text: LocaleText.tr("Delete")
                    onTriggered: {
                      root.deleteDialogObject.ticketId = modelData.ticket.id
                      root.deleteDialogObject.ticketName = modelData.ticket.name
                      root.deleteDialogObject.open()
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
