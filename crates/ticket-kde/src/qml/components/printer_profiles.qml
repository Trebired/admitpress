pragma Singleton

import QtQml

QtObject {
  function fetch(root, backend) {
    const data = root.parseResponse(backend.printerProfiles())
    return data.ok && Array.isArray(data.data.profiles) ? data.data.profiles : []
  }

  function find(profiles, queue) {
    return profiles.find(function(entry) {
        return entry.queue === queue
    }) || null
  }
}
