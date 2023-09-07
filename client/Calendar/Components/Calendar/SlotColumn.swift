//
//  SlotColumn.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/5/23.
//

import SwiftUI

struct SlotColumn: View {

  var body: some View {
    VStack(spacing: 0) {
      ToolbarItem {
        Color.clear
      }

      Color.clear.frame(height: 18)

      LazyVStack(spacing: 0) {
        Grid(verticalSpacing: 0) {
          ForEach(DateEnumarator.hours, id: \.hour) { hour in
            GridRow {
              ZStack {
                Color.clear

                Text(hour.toFormat("hh a").lowercased())
                  .font(.caption2)
                  .foregroundColor(.gray)
              }
              .frame(height: Constants.WeekSlotHeight)
            }
          }
        }
      }
      .frame(width: Constants.SlotWidth)
    }
  }
}

struct SlotColumn_Previews: PreviewProvider {

  static var previews: some View {
    SlotColumn()
  }
}
