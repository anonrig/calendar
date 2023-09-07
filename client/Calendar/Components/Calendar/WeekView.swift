//
//  WeekView.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/7/23.
//

import SwiftUI

struct WeekView: View {

  var body: some View {
    ScrollView(.vertical, showsIndicators: false) {
      HStack(alignment: .top, spacing: 0) {
        SlotColumn()

        LazyHStack(spacing: 0) {
          ForEach(DateEnumarator.daysThisWeek(), id: \.self) { date in
            DayColumn(date: date)
          }
        }
      }
    }
  }
}

struct WeekView_Previews: PreviewProvider {

  static var previews: some View {
    WeekView()
  }
}
