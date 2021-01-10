import 'package:flutter/material.dart';
import 'package:flutter_app/branch/utils/location_helper.dart';
import 'package:mapbox_gl/mapbox_gl.dart';

class MapsPage extends StatelessWidget {
  static Route<dynamic> route() => MaterialPageRoute(
        builder: (context) => MapsPage(),
      );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Maps"),
      ),
      body: MapboxMap(
        accessToken:
            'pk.eyJ1IjoiaW52aWN0YTExMSIsImEiOiJja2hyMmNmemYwejJjMnpwNTNma2t5aXFpIn0.y5-1daPfr2gL1o51q6QHJA',
        styleString: "mapbox://styles/invicta111/ckhr495nz00ay1amkd1bjstli",
        initialCameraPosition: CameraPosition(target: LatLng(45.45, 45.45)),
        onMapCreated: (MapboxMapController controller) async {
          final location = await acquireCurrentLocation();
          final animateCameraResult = await controller.animateCamera(
            CameraUpdate.newCameraPosition(
              CameraPosition(
                zoom: 15.0,
                target: location,
              ),
            ),
          );
          if (animateCameraResult) {
            controller.addCircle(
              CircleOptions(
                circleRadius: 10.0,
                circleColor: '#E5FFCC',
                geometry: location,
              ),
            );
          }
        },
      ),
    );
  }
}
