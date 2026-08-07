import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../../../../components"

AppDialog {
  id: ticketArchiveExportSelectDialog

  title: LocaleText.tr("Export ticket")
  dialogWidth: 460

  function archiveTickets() {
    return root && root.appState && root.appState.tickets ? root.appState.tickets : []
  }

  function archiveTicketNames() {
    return archiveTickets().map(function(summary) {
        return summary.ticket.name
    })
  }

  function selectedTicket() {
    const tickets = archiveTickets()
    if (exportTicketCombo.currentIndex < 0 || exportTicketCombo.currentIndex >= tickets.length) {
      return null
    }
    return tickets[exportTicketCombo.currentIndex].ticket
  }

  contentItem: ColumnLayout {
    spacing: Kirigami.Units.smallSpacing

    Controls.Label {
      text: LocaleText.tr("Ticket")
    }

    Controls.ComboBox {
      id: exportTicketCombo
      Layout.fillWidth: true
      model: ticketArchiveExportSelectDialog.archiveTicketNames()
      enabled: count > 0
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Export...")
        icon.name: "document-export"
        highlighted: true
        enabled: ticketArchiveExportSelectDialog.selectedTicket() !== null
        onClicked: {
          const ticket = ticketArchiveExportSelectDialog.selectedTicket()
          if (!ticket) return
          ticketArchiveExportSelectDialog.close()
          root.ticketArchiveExportDialogObject.openFor(ticket.id, ticket.name)
        }
      }
    }
  }
}
