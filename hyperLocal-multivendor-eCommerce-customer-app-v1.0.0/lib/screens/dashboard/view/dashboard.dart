import 'dart:developer';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:go_router/go_router.dart';
import 'package:heroicons_flutter/heroicons_flutter.dart';
import 'package:remixicon/remixicon.dart';
import '../../../bloc/user_cart_bloc/user_cart_bloc.dart';
import '../../../bloc/user_cart_bloc/user_cart_state.dart';
import '../../../services/location/location_service.dart';
import '../../../l10n/app_localizations.dart';
import '../../../services/cart_service.dart';
import '../../../utils/widgets/custom_toast.dart';

class Dashboard extends StatefulWidget {
  final int index;
  final StatefulNavigationShell navigationShell;
  const Dashboard({
    super.key,
    required this.index,
    required this.navigationShell
  });

  @override
  State<Dashboard> createState() => _DashboardState();
}

class _DashboardState extends State<Dashboard> {
  bool showBottomNavBar = true;
  late int _currentIndex;
  DateTime? _lastBackPressed;
  final controller = PageController();

  @override
  void initState() {
    super.initState();
    _currentIndex = widget.index;
    // Debug: Print stored location
    final storedLocation = LocationService.getStoredLocation();
    if (storedLocation != null) {
      log('Stored Location: ${storedLocation.fullAddress}');
      log('Area: ${storedLocation.area}');
      log('City: ${storedLocation.city}');
      log('State: ${storedLocation.state}');
    } else {
      log('No location stored in Hive');
    }
  }

  void _goBranch(int index) {
    _currentIndex = index;
    widget.navigationShell.goBranch(index);
    setState(() {});
  }

  Future<void> _handleBack(BuildContext context) async {
    if (widget.index != 0) {
      _goBranch(0);
      return;
    }

    final now = DateTime.now();
    if (_lastBackPressed == null ||
        now.difference(_lastBackPressed!) > const Duration(seconds: 2)) {
      _lastBackPressed = now;
      ToastManager.show(
        context: context,
        message: AppLocalizations.of(context)?.pressAgainToExitTheApp ?? 'Press again to exit the app',
      );
      return;
    }

    SystemNavigator.pop();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    return BlocListener<CartBloc, CartState>(
      listener: (context, state) {
        if (state is CartLoaded) {
          if(state.errorMessage != null){
            ToastManager.show(
              context: context,
              message: state.errorMessage ?? 'Failed to add item to cart',
              type: ToastType.error,
            );
          }
        }
          // context.read<GetUserCartBloc>().add(FetchUserCart());
          CartService.triggerCartAnimationOnFirstAdd(context, state);
      },
      child: PopScope(
        canPop: false,
        onPopInvokedWithResult: (bool didPop, Object? result) async {
          if (didPop) return;
          await _handleBack(context);
        },
        child: Scaffold(
          body: widget.navigationShell,

          bottomNavigationBar: Container(
            height: 70.h,
            decoration: BoxDecoration(
              border: Border(
                top: BorderSide(
                  color: Theme.of(context).colorScheme.outlineVariant,
                  width: 1.0
                )
              ),
            ),
            child: BottomNavigationBar(
              type: BottomNavigationBarType.fixed,
              currentIndex: _currentIndex,
              selectedItemColor: Theme.of(context).colorScheme.tertiary,
              unselectedItemColor: Theme.of(context).colorScheme.tertiary.withValues(alpha: 0.8),
              unselectedLabelStyle: TextStyle(
                fontWeight: FontWeight.w600,
                fontSize: 12
              ),
              selectedLabelStyle: TextStyle(
                  fontWeight: FontWeight.bold,
                fontSize: 13
              ),
              onTap: _goBranch,
              items: [
                BottomNavigationBarItem(
                  icon: Icon(widget.index == 0 ? RemixIcons.home_smile_fill : RemixIcons.home_smile_line),
                  label: l10n?.home ?? 'Home',
                ),
                BottomNavigationBarItem(
                  icon: Icon(widget.index == 1 ? HeroiconsSolid.squares2x2 : HeroiconsOutline.squares2x2),
                  label: l10n?.categories ?? 'Categories',
                ),
                BottomNavigationBarItem(
                  icon: Icon(widget.index == 2 ? HeroiconsSolid.buildingStorefront : HeroiconsOutline.buildingStorefront),
                  label: l10n?.stores ?? 'Stores',
                ),
                BottomNavigationBarItem(
                      icon: Icon(widget.index == 3 ? HeroiconsSolid.userCircle : HeroiconsOutline.userCircle),
                  label: l10n?.account ?? 'Account',
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    // TODO: implement dispose
    controller.dispose();
    super.dispose();
  }
}
