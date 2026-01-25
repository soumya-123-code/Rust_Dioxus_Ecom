import 'dart:developer';
import 'dart:io';

import 'package:dio/dio.dart';
import 'package:hyper_local/config/api_base_helper.dart';
import 'package:hyper_local/config/constant.dart';
import 'package:image_picker/image_picker.dart';
import 'package:path_provider/path_provider.dart';
import 'package:permission_handler/permission_handler.dart';

import '../../../config/api_routes.dart';
import '../model/order_detail_model.dart';
import '../model/delivery_tracking_model.dart';

class OrderRepository {
  Future<Map<String, dynamic>> createOrder({
    required String paymentType,
    required String promoCode,
    required String giftCard,
    required int addressId,
    required bool rushDelivery,
    required bool useWallet,
    required String orderNote,
    Map<String, dynamic>? paymentDetails
  }) async {
    try {
      String? paymenttype;
      if(paymentType.isNotEmpty && paymentType != 'wallet'){
        paymenttype = paymentType == 'cod' ? paymentType : '${paymentType}Payment';
      } else if(paymentType == 'wallet') {
        paymenttype = paymentType;
      } else {
        paymenttype = '';
      }
        final response = await AppConstant.apiBaseHelper.postAPICall(
            ApiRoutes.createOrderApi,
            {
              'payment_type': paymenttype,
              'promo_code': promoCode,
              'gift_card': giftCard,
              'address_id': addressId,
              'rush_delivery': rushDelivery,
              'use_wallet': useWallet,
              'order_note': orderNote,
              if(paymentType != 'flutterwave')
                'redirect_url': AppConstant.baseUrl,
              ...paymentType.toLowerCase() != 'cod' && paymentDetails != null
                  ? paymentDetails
                  : {},
            }
        );
        if (response.statusCode == 200) {
          return response.data;
        } else {
          return {};
        }
    } catch (e) {
      throw ApiException(e.toString());
    }
  }

  Future<Map<String, dynamic>> fetchMyOrderList({required int perPage, required int page}) async {
    try{
      final response = await AppConstant.apiBaseHelper.getAPICall(
        '${ApiRoutes.getMyOrderApi}?page=$page&per_page=$perPage',
        {}
      );
      if(response.statusCode == 200 ){
        return response.data;
      }
      return {};
    }catch(e) {
      throw ApiException('Failed to get my orders list');
    }
  }

  Future<List<OrderDetailModel>> getOrderDetail({required String orderSlug,}) async {
    try{
      final response = await AppConstant.apiBaseHelper.getAPICall(
        ApiRoutes.orderDetailApi+orderSlug,
        {},
      );

      if(response.statusCode == 200) {
        final List<OrderDetailModel> orderData = [];
        orderData.add(OrderDetailModel.fromJson(response.data));
        return orderData;
      } else {
        return [];
      }

    }catch(e){
      throw ApiException(e.toString());
    }
  }

  Future<DeliveryBoyTrackingModel?> getDeliveryTracking({required String orderSlug,}) async {
    try{
      final response = await AppConstant.apiBaseHelper.getAPICall(
        '${ApiRoutes.orderDetailApi}$orderSlug/delivery-boy-location',
        {},
      );

      if(response.statusCode == 200) {
        return DeliveryBoyTrackingModel.fromJson(response.data);
      } else {
        return null;
      }
    }catch(e){
      throw ApiException(e.toString());
    }
  }

  Future<String> downloadInvoicePdf(String invoiceUrl) async {
    try {
      final response = await AppConstant.apiBaseHelper.getAPICall(
        invoiceUrl,
        {}
      );
      if(response.data != null) {
        if (Platform.isAndroid) {
          await Permission.storage.request();
        }
        // Get the appropriate directory
        Directory? directory;
        if (Platform.isAndroid) {
          // For Android - use Downloads directory
          directory = Directory('/storage/emulated/0/Download');
          if (!await directory.exists()) {
            directory = await getExternalStorageDirectory();
          }
        } else if (Platform.isIOS) {
          // For iOS - use Documents directory (accessible in Files app)
          directory = await getApplicationDocumentsDirectory();
        }
        final fileName = 'invoice_${DateTime.now().millisecondsSinceEpoch}.pdf';
        final filePath = '${directory!.path}/$fileName';
        await AppConstant.apiBaseHelper.downloadFile(
          url: invoiceUrl,
          cancelToken: CancelToken(),
          savePath: filePath,
          updateDownloadedPercentage: (received, total) { // Two parameters
            if (total != -1) {
              final percentage = (received / total * 100);
              log('Download: ${percentage.toStringAsFixed(0)}%');
            }
          },
        );
        return filePath;
      } else {
        return '';
      }
    } catch (e) {
      throw ApiException('Failed to download invoice: $e');
    }
  }

  Future<Map<String, dynamic>> returnOrderItemRequest({
    required int orderItemId,
    required String reason,
    List<XFile> images = const [],
  }) async {
    try{

      final form = await formDataWithImages(
        fields: {
          'reason': reason,
        },
        images: images,
        imageFieldLabel: 'images'
      );

      log('Return Order Item Request ${form.files}');

      final response = await AppConstant.apiBaseHelper.postAPICall(
        '${ApiRoutes.returnOrderItemApi}$orderItemId/return',
        form
      );
      if(response.statusCode == 200) {
        return response.data;
      }
      return {};
    }catch(e) {
      throw ApiException(e.toString());
    }
  }

  Future<Map<String, dynamic>> cancelReturnRequest({
    required int orderItemId,
  }) async {
    try{
      final response = await AppConstant.apiBaseHelper.postAPICall(
          '${ApiRoutes.cancelReturnRequestApi}$orderItemId/return-cancel',
          {}
      );

      if(response.statusCode == 200) {
        return response.data;
      }
      return {};
    }catch(e) {
      throw ApiException(e.toString());
    }
  }

  Future<Map<String, dynamic>> cancelOrderItem({
    required int orderItemId,
  }) async {
    try{
      final response = await AppConstant.apiBaseHelper.postAPICall(
          '${ApiRoutes.cancelOrderItemApi}$orderItemId/cancel',
          {}
      );

      if(response.statusCode == 200) {
        return response.data;
      }
      return {};
    }catch(e) {
      throw ApiException(e.toString());
    }
  }
}