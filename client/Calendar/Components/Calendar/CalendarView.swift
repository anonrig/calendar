//
//  CalendarView.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/5/23.
//

import SwiftUI

struct CalendarView: View {

  var body: some View {
    WeekView()
  }
}

struct CalendarView_Previews: PreviewProvider {

  static var previews: some View {
    CalendarView()
      .frame(width: 1080)
  }
}
