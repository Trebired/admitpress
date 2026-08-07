import QtQuick

Item {
  id: backgroundTasks

  property var root
  property var backend

  signal updateDetected(var release)
  readonly property alias backups: backupScheduler
  readonly property alias release: releaseWatcher

  function handleResponse(operation, responseJson) {
    if (operation === "check_release") {
      releaseWatcher.handleResponse(responseJson)
      return true
    }
    if (operation === "backup_now" || operation === "backup_if_due") {
      backupScheduler.handleResponse(operation, responseJson)
      return true
    }
    return false
  }

  BackupScheduler {
    id: backupScheduler
    root: backgroundTasks.root
    backend: backgroundTasks.backend
  }

  ReleaseWatcher {
    id: releaseWatcher
    root: backgroundTasks.root
    backend: backgroundTasks.backend
    onUpdateDetected: function(release) {
      backgroundTasks.updateDetected(release)
    }
  }
}
