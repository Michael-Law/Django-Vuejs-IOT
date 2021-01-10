import 'package:flutter/material.dart';
import 'package:flutter_app/branch/pages/home_page.dart';
import 'package:flutter_app/branch/pages/account.dart';
import 'package:flutter_app/branch/pages/maps.dart';

class TabNavigationItem {
  final Widget page;
  final Widget title;
  final Icon icon;

  TabNavigationItem({
    @required this.page,
    @required this.title,
    @required this.icon,
  });

  static List<TabNavigationItem> get items => [
        TabNavigationItem(
          page: HomePage(),
          icon: Icon(Icons.home),
          title: Text("Home"),
        ),
        TabNavigationItem(
          page: AccountPage(),
          icon: Icon(Icons.account_balance_wallet_sharp),
          title: Text("Account"),
        ),
        TabNavigationItem(
          page: MapsPage(),
          icon: Icon(Icons.map_sharp),
          title: Text("Map"),
        ),
      ];
}
