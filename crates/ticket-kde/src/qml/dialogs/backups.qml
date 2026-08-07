import QtQuick
import QtQuick.Controls as Controls
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import "../components"

AppDialog {
  id: backupsDialog

  readonly property var scheduler: root ? root.backupSchedulerObject : null
  readonly property var status: scheduler ? scheduler.status : null

  title: LocaleText.tr("Backups")
  dialogWidth: 560

  function lastBackupText() {
    const stamp = status ? status.last_backup_at : ""
    if (!stamp) return LocaleText.tr("No backup has been made yet.")
    return TextFormat.stampDateTime(stamp)
  }

  function storedText() {
    const count = status ? status.count : 0
    const megabytes = (status ? status.total_bytes : 0) / (1024 * 1024)
    return count + " (" + megabytes.toFixed(1) + " MB)"
  }

  onOpened: if (scheduler) scheduler.refreshStatus()

  contentItem: ColumnLayout {
    spacing: backupsDialog.contentSpacing

    Controls.Label {
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      text: LocaleText.tr("Every three hours the database and the ticket artwork are packed into "
        + "an archive. Copies older than a month are removed automatically.")
    }

    SectionBorder {}

    GridLayout {
      Layout.fillWidth: true
      columns: 2
      rowSpacing: Kirigami.Units.smallSpacing
      columnSpacing: Kirigami.Units.largeSpacing

      Controls.Label {
        text: LocaleText.tr("Last backup")
        color: Kirigami.Theme.disabledTextColor
      }
      Controls.Label {
        Layout.fillWidth: true
        text: backupsDialog.lastBackupText()
      }

      Controls.Label {
        text: LocaleText.tr("Stored copies")
        color: Kirigami.Theme.disabledTextColor
      }
      Controls.Label {
        Layout.fillWidth: true
        text: backupsDialog.storedText()
      }

      Controls.Label {
        Layout.alignment: Qt.AlignTop
        text: LocaleText.tr("Folder")
        color: Kirigami.Theme.disabledTextColor
      }
      Controls.Label {
        Layout.fillWidth: true
        wrapMode: Text.WrapAnywhere
        text: backupsDialog.status ? backupsDialog.status.directory : ""
      }
    }

    Kirigami.InlineMessage {
      Layout.fillWidth: true
      type: Kirigami.MessageType.Error
      visible: backupsDialog.scheduler && backupsDialog.scheduler.lastError !== ""
      text: backupsDialog.scheduler ? backupsDialog.scheduler.lastError : ""
    }

    DialogActions {
      Controls.Button {
        text: LocaleText.tr("Back up now")
        icon.name: AppIcons.pick("backup")
        highlighted: true
        enabled: backupsDialog.scheduler && !backupsDialog.scheduler.working
        onClicked: backupsDialog.scheduler.backupNow()
      }
    }
  }
}
