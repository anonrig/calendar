//
//  DateEnumerator.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/5/23.
//

import SwiftUI
import SwiftDate

struct DateEnumarator {
  static let hours: [Date] = {
    let startDate: Date = Date().dateAt(.startOfDay).dateBySet(hour: 0, min: 0, secs: 0)!
    var dates = [startDate]
    var iteratingDate = startDate

    repeat {
      iteratingDate = iteratingDate + 1.hours
      dates.append(iteratingDate)
    } while (iteratingDate.compare(.isSameDay(startDate)))

    return dates
  }()

  static func daysThisWeek() -> [Date] {
    let startDate: Date = Date().dateAt(.startOfWeek).dateBySet(hour: 0, min: 0, secs: 0)!
    var dates = [startDate]
    var iteratingDate = startDate

    repeat {
      iteratingDate = iteratingDate + 1.days
      dates.append(iteratingDate)
    } while (iteratingDate.compare(.isSameWeek(startDate)))

    return dates
  }
}
