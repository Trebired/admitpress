import QtQuick
import "../components"

QtObject {
  id: days

  required property var root
  required property var backend
  required property var page

  function dayBreakdownText(day) {
    if (!day || !day.subtickets || day.subtickets.length === 0) return LocaleText.tr("No sales")
    const parts = []
    for (let subticket of day.subtickets) {
      const currency = subticket.currency || root.appState.main_currency || ""
      const label = MoneyFormat.format(subticket.amount_minor, currency)
      parts.push(subticket.quantity + "x " + label)
    }
    return parts.join(" | ")
  }

  function daySummaries() {
    return page.summary && page.summary.day_summaries ? page.summary.day_summaries : []
  }

  function hasSales(day) {
    return Number(day.ticket_count || 0) > 0
  }

  function visibleDaySummaries() {
    if (page.showEmptyDays) return daySummaries()
    return daySummaries().filter(hasSales)
  }

  function selectableDaySummaries() {
    return daySummaries().filter(hasSales)
  }

  function hasSelectableDays() {
    return selectableDaySummaries().length > 0
  }

  function monthKey(localDate) {
    return String(localDate || "").substring(0, 7)
  }

  function monthLabel(monthKeyValue) {
    const parts = String(monthKeyValue || "").split("-")
    const monthIndex = Number(parts[1] || 1) - 1
    return LocaleText.monthName(monthIndex) + " " + (parts[0] || "")
  }

  function monthGroups() {
    const groups = []
    const index = {}
    for (let day of daySummaries()) {
      const key = monthKey(day.local_date)
      let group = index[key]
      if (!group) {
        group = { key: key, days: [], dayCount: 0, ticketTotal: 0, moneyTotal: 0, currency: "" }
        index[key] = group
        groups.push(group)
      }
      group.days.push(day)
      const ticketCount = Number(day.ticket_count || 0)
      if (ticketCount > 0) {
        group.dayCount += 1
        group.ticketTotal += ticketCount
        group.moneyTotal += Number(day.total_minor || 0)
        const currency = day.currency || root.appState.main_currency || ""
        if (!group.currency && currency) group.currency = currency
      }
    }
    return groups.filter(function(group) {
        return group.ticketTotal > 0
    })
  }

  function visibleMonthGroups() {
    const groups = monthGroups()
    if (page.showEmptyDays) return groups
    return groups.map(function(group) {
        return Object.assign({}, group, {
            days: group.days.filter(function(day) {
                return Number(day.ticket_count || 0) > 0
            })
        })
    })
  }

  function monthDays(monthKeyValue) {
    return selectableDaySummaries().filter(function(day) {
        return monthKey(day.local_date) === monthKeyValue
    })
  }

  function monthSelectionState(monthKeyValue) {
    page.selectedDayRevision
    const days = monthDays(monthKeyValue)
    if (days.length === 0) return Qt.Unchecked
    let selected = 0
    for (let day of days) {
      if (page.selectedDayMap[day.local_date]) selected += 1
    }
    if (selected === 0) return Qt.Unchecked
    return selected === days.length ? Qt.Checked : Qt.PartiallyChecked
  }

  function setMonthSelected(monthKeyValue, selected) {
    const next = Object.assign({}, page.selectedDayMap)
    for (let day of monthDays(monthKeyValue)) {
      if (selected) next[day.local_date] = true
      else delete next[day.local_date]
    }
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function allSelectionState() {
    page.selectedDayRevision
    const summaries = selectableDaySummaries()
    if (summaries.length === 0) return Qt.Unchecked
    let selected = 0
    for (let day of summaries) {
      if (page.selectedDayMap[day.local_date]) selected += 1
    }
    if (selected === 0) return Qt.Unchecked
    return selected === summaries.length ? Qt.Checked : Qt.PartiallyChecked
  }

  function lockedDateMap() {
    const today = backend.today()
    const dates = page.summary && page.summary.sales_dates ? page.summary.sales_dates : []
    const map = {}
    for (let localDate of dates) {
      if (String(localDate) !== today) map[String(localDate)] = true
    }
    return map
  }

  function daySelected(localDate) {
    page.selectedDayRevision
    return !!page.selectedDayMap[localDate]
  }

  function setDaySelected(localDate, selected) {
    const next = Object.assign({}, page.selectedDayMap)
    if (selected) next[localDate] = true
    else delete next[localDate]
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function selectAllDays() {
    const next = {}
    for (let day of selectableDaySummaries()) next[day.local_date] = true
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function clearSelectedDays() {
    page.selectedDayMap = ({})
    page.selectedDayRevision += 1
  }

  function selectedDayCount() {
    page.selectedDayRevision
    let count = 0
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) count += 1
    }
    return count
  }

  function selectedDayTicketCount() {
    page.selectedDayRevision
    let count = 0
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) count += Number(day.ticket_count || 0)
    }
    return count
  }

  function selectedDayTotalMinor() {
    page.selectedDayRevision
    let total = 0
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) total += Number(day.total_minor || 0)
    }
    return total
  }

  function selectedDayCurrency() {
    page.selectedDayRevision
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) return day.currency || ""
    }
    return ""
  }

  function selectedDaySalesRows() {
    page.selectedDayRevision
    const rows = []
    const index = {}
    for (let day of selectableDaySummaries()) {
      if (!page.selectedDayMap[day.local_date]) continue
      for (let subticket of (day.subtickets || [])) {
        const currency = subticket.currency || root.appState.main_currency || ""
        const key = [
          subticket.subticket_id || "",
          subticket.amount_minor || 0,
          currency
        ].join(":")
        let row = index[key]
        if (!row) {
          row = {
            subticket_id: subticket.subticket_id,
            name: subticket.name,
            display_label: subticket.display_label,
            amount_minor: subticket.amount_minor,
            currency: currency,
            quantity: 0,
            total_minor: 0
          }
          index[key] = row
          rows.push(row)
        }
        row.quantity += Number(subticket.quantity || 0)
        row.total_minor += Number(subticket.total_minor || 0)
      }
    }
    return rows
  }

  function selectedResetDay() {
    const dates = selectedDayDates()
    return dates.length === 1 ? dates[0] : ""
  }

  function canResetSelectedDay() {
    const localDate = selectedResetDay()
    return localDate !== "" && page.isDayLocked && !page.isDayLocked(localDate)
  }

  function cleanupSelectedDays() {
    const next = {}
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) next[day.local_date] = true
    }
    page.selectedDayMap = next
    page.selectedDayRevision += 1
  }

  function switchToDate(localDateValue) {
    if (page.isDayLocked && page.isDayLocked(localDateValue)) return
    page.localDate = localDateValue
    page.load()
  }

  function selectedDayDates() {
    page.selectedDayRevision
    const dates = []
    for (let day of selectableDaySummaries()) {
      if (page.selectedDayMap[day.local_date]) dates.push(day.local_date)
    }
    return dates
  }
}
