<?php

namespace App\Http\Requests\User\Order;

use App\Enums\Payment\PaymentTypeEnum;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Validation\Rule;

class CreateOrderRequest extends FormRequest
{
    /**
     * Determine if the user is authorized to make this request.
     */
    public function authorize(): bool
    {
        return true;
    }

    /**
     * Get the validation rules that apply to the request.
     *
     * @return array<string, \Illuminate\Contracts\Validation\ValidationRule|array<mixed>|string>
     */
    public function rules(): array
    {
        $rules = [
            'payment_type' => ['required', Rule::in(PaymentTypeEnum::values())],
            'promo_code' => ['nullable', 'string', 'max:50'],
            'gift_card' => ['nullable', 'string', 'max:50'],
            'address_id' => ['required', 'numeric', 'exists:addresses,id'],
            'rush_delivery' => ['boolean', 'nullable'],
            'use_wallet' => ['boolean', 'nullable'],
            'order_note' => ['nullable', 'string', 'max:500'],
            'redirect_url' => ['nullable'],
        ];

        if (in_array($this->input('payment_type'), [PaymentTypeEnum::STRIPE(), PaymentTypeEnum::RAZORPAY(), PaymentTypeEnum::PAYSTACK()])) {
            $rules['transaction_id'] = ['required', 'string'];
        }
        if (!empty($this->input('redirect_url'))) {
            $rules['redirect_url'] = ['required', 'url'];
        }
        if ($this->input('payment_type') === PaymentTypeEnum::RAZORPAY()) {
            $rules['razorpay_order_id'] = ['required', 'string'];
            $rules['razorpay_signature'] = ['required', 'string'];
        }

        return $rules;
    }
}
