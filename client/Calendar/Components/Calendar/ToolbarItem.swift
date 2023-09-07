//
//  ToolbarItem.swift
//  Calendar
//
//  Created by Yagiz Nizipli on 9/5/23.
//

import SwiftUI

struct ToolbarItem<Content: View>: View {

  let content: Content

  init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content()
  }

  var body: some View {
    ZStack {
      Color.clear

      content.frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    .frame(width: 88, height: Constants.WeekSlotHeight)
  }
}

struct ToolbarItem_Previews: PreviewProvider {

  static var previews: some View {
    ToolbarItem {
      Text("hello")
    }
  }
}
