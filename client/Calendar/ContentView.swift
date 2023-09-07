//
//  ContentView.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/4/23.
//

import SwiftUI

struct ContentView: View {
  var body: some View {
    NavigationSplitView {
      Text("sidebar")
        .frame(minWidth: 150, idealWidth: 200)
    } detail: {
      CalendarView()
    }
  }
}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView()
  }
}
