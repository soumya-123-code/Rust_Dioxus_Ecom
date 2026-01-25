<?php

namespace App\Services;

use App\Models\UserFcmToken;
use Kreait\Firebase\Contract\Messaging;
use Kreait\Firebase\Factory;
use Kreait\Firebase\Messaging\CloudMessage;
use Kreait\Firebase\Messaging\Notification;
use Illuminate\Support\Collection;

class FirebaseService
{
    protected Messaging $messaging;

    public function __construct()
    {
        $firebase = (new Factory)->withServiceAccount(config('services.firebase.credentials.file'));
        $this->messaging = $firebase->createMessaging();
    }

    public function sendNotification($token, $title, $body, $image = "", $data = []): array
    {
        $notification = Notification::create(title: $title, body: $body, imageUrl: $image);
        $message = CloudMessage::new()
            ->withNotification($notification)
            ->withData($data)
            ->withDefaultSounds()
            ->toToken($token)
            // ->toTopic('...')
            // ->toCondition('...')
        ;

        return $this->messaging->send($message);
    }

    /**
     * Send a notification to multiple tokens in chunks
     */
    public function sendBulkNotification(array $tokens, string $title, string $body, string $image = null, array $data = [], int $chunkSize = 50): array
    {
        $results = [
            'success' => 0,
            'failure' => 0,
            'responses' => [],
        ];

        $notification = Notification::create(title: $title, body: $body, imageUrl: $image);
        $results['removed_tokens'] = [];
        // Convert to Laravel collection for easy chunking
        Collection::make($tokens)->chunk($chunkSize)->each(function ($chunk) use (&$results, $notification, $data) {
            $message = CloudMessage::new()
                ->withNotification($notification)
                ->withDefaultSounds()
                ->withData($data);

            $multicastResult = $this->messaging->sendMulticast($message, $chunk->toArray());

            // Count results
            $results['success'] += $multicastResult->successes()->count();
            $results['failure'] += $multicastResult->failures()->count();

            // Get invalid tokens
            $invalidTokens = $multicastResult->invalidTokens();

            if (!empty($invalidTokens)) {
                UserFcmToken::whereIn('fcm_token', $invalidTokens)->delete();
                $results['removed_tokens'] = array_merge($results['removed_tokens'], $invalidTokens);
            }
        });

        return $results;
    }
}
