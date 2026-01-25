import 'package:equatable/equatable.dart';

abstract class SaveForLaterEvent extends Equatable {
  @override
  // TODO: implement props
  List<Object?> get props => [];
}

class FetchSavedProducts extends SaveForLaterEvent {}

class FetchMoreSavedProducts extends SaveForLaterEvent {}


class SaveForLaterRequest extends SaveForLaterEvent {
  final int cartItemId;

  SaveForLaterRequest({required this.cartItemId});
  @override
  // TODO: implement props
  List<Object?> get props => [cartItemId];
}
