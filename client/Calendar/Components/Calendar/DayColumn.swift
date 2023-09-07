//
//  DayColumn.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/4/23.
//

import SwiftUI
import SwiftDate

struct DayColumn: View {

  var date: Date
  let hourHeight = 120

  var body: some View {
    VStack(spacing: 0) {
      ToolbarItem {
        ZStack(alignment: .center) {
          Group {
            if date.compare(.isToday) {
              VStack {
                Color.red.frame(width: 16, height: 2)

                Spacer()
              }
            } else {
              Color.clear
            }
          }
          .frame(maxWidth: .infinity)

          Text(date.toFormat("EEE d"))
            .font(.headline)
            .fontWeight(.semibold)
            .foregroundColor(date.compare(.isToday) ? .red : .gray)
        }
      }

      LazyVStack(spacing: 0) {
        Grid(verticalSpacing: 0) {
          ForEach(DateEnumarator.hours, id: \.hour) { hour in
            GridRow {
              ZStack {
                Color.white

                Text(hour.toFormat("hh:mm a"))
              }
              .frame(width: 115, height: Constants.WeekSlotHeight)
            }

            Divider()
          }
        }
      }
    }
  }
}

struct DayColumn_Previews: PreviewProvider {

  static var previews: some View {
    HStack(spacing: 0) {
      Spacer()
      DayColumn(date: .now)
      DayColumn(date: Date.now + 1.days)
      Spacer()
    }

    .frame(width: 230, height: 1080)
  }
}
