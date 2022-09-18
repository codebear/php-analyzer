<?php
$OVERRIDE = [
    'classes' => [
        'DOMDocument' => [
            'methods' => [
                'createElement' => [
                    'return' => ['\\DOMElement']
                ]
            ]
        ]
    ]
];
