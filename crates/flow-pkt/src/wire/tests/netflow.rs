// Copyright (C) 2023-present The NetGauze Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr},
};

use chrono::{TimeZone, Utc};
use netgauze_iana::tcp::*;
use netgauze_parse_utils::{test_helpers::*, ReadablePduWithOneInput, Span};

use crate::{
    netflow::*,
    wire::{
        deserializer::netflow::{
            LocatedNetFlowV9PacketParsingError, NetFlowV9PacketParsingError, SetParsingError,
        },
        serializer::netflow::*,
    },
    *,
};

#[test]
fn test_netflow9_template_record() -> Result<(), NetFlowV9WritingError> {
    let good_wire = [
        0x00, 0x09, 0x00, 0x01, 0x00, 0x06, 0x14, 0x8b, 0x59, 0x77, 0x3e, 0x3d, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x04, 0x00, 0x00, 0x0d, 0x00, 0x08,
        0x00, 0x04, 0x00, 0x0c, 0x00, 0x04, 0x00, 0x15, 0x00, 0x04, 0x00, 0x16, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x04, 0x00, 0x0a, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x04,
        0x00, 0x07, 0x00, 0x02, 0x00, 0x0b, 0x00, 0x02, 0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x3c, 0x00, 0x01,
    ];

    let good = NetFlowV9Packet::new(
        398475,
        Utc.with_ymd_and_hms(2017, 7, 25, 12, 49, 1).unwrap(),
        0,
        0,
        vec![Set::Template(vec![TemplateRecord::new(
            1024,
            vec![
                FieldSpecifier::new(IE::sourceIPv4Address, 4).unwrap(),
                FieldSpecifier::new(IE::destinationIPv4Address, 4).unwrap(),
                FieldSpecifier::new(IE::flowEndSysUpTime, 4).unwrap(),
                FieldSpecifier::new(IE::flowStartSysUpTime, 4).unwrap(),
                FieldSpecifier::new(IE::octetDeltaCount, 4).unwrap(),
                FieldSpecifier::new(IE::packetDeltaCount, 4).unwrap(),
                FieldSpecifier::new(IE::ingressInterface, 4).unwrap(),
                FieldSpecifier::new(IE::egressInterface, 4).unwrap(),
                FieldSpecifier::new(IE::sourceTransportPort, 2).unwrap(),
                FieldSpecifier::new(IE::destinationTransportPort, 2).unwrap(),
                FieldSpecifier::new(IE::protocolIdentifier, 1).unwrap(),
                FieldSpecifier::new(IE::tcpControlBits, 1).unwrap(),
                FieldSpecifier::new(IE::ipVersion, 1).unwrap(),
            ],
        )])],
    );
    let mut templates_map = HashMap::new();
    test_parsed_completely_with_one_input(&good_wire, &mut templates_map, &good);
    test_write_with_two_inputs(&good, None, true, &good_wire)?;
    Ok(())
}

#[test]
fn test_netflow9_data_record() -> Result<(), NetFlowV9WritingError> {
    let good_wire = [
        0x00, 0x09, 0x00, 0x04, 0x00, 0x06, 0xfe, 0xc9, 0x59, 0x77, 0x3e, 0x79, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0xa0, 0xc0, 0xa8, 0x01, 0x64, 0xd8, 0x3a,
        0xd3, 0x63, 0x00, 0x01, 0xa2, 0xa5, 0x00, 0x01, 0xa1, 0xec, 0x00, 0x00, 0x00, 0x42, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0x85, 0x01, 0xbb,
        0x11, 0x00, 0x04, 0xd8, 0x3a, 0xd3, 0x63, 0xc0, 0xa8, 0x01, 0x64, 0x00, 0x01, 0xa2, 0xa5,
        0x00, 0x01, 0xa1, 0xec, 0x00, 0x00, 0x05, 0x62, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xbb, 0xcc, 0x85, 0x11, 0x00, 0x04, 0xc0, 0xa8, 0x01,
        0x64, 0xd8, 0x3a, 0xd3, 0x6e, 0x00, 0x01, 0xcb, 0x55, 0x00, 0x01, 0xcb, 0x55, 0x00, 0x00,
        0x00, 0x42, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6,
        0x87, 0x01, 0xbb, 0x11, 0x00, 0x04, 0xc0, 0xa8, 0x01, 0x64, 0xd8, 0x3a, 0xd3, 0x6e, 0x00,
        0x02, 0x38, 0x75, 0x00, 0x02, 0x38, 0x75, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf7, 0x29, 0x01, 0xbb, 0x11, 0x00, 0x04,
    ];

    let fields = vec![
        FieldSpecifier::new(IE::sourceIPv4Address, 4).unwrap(),
        FieldSpecifier::new(IE::destinationIPv4Address, 4).unwrap(),
        FieldSpecifier::new(IE::flowEndSysUpTime, 4).unwrap(),
        FieldSpecifier::new(IE::flowStartSysUpTime, 4).unwrap(),
        FieldSpecifier::new(IE::octetDeltaCount, 4).unwrap(),
        FieldSpecifier::new(IE::packetDeltaCount, 4).unwrap(),
        FieldSpecifier::new(IE::ingressInterface, 4).unwrap(),
        FieldSpecifier::new(IE::egressInterface, 4).unwrap(),
        FieldSpecifier::new(IE::sourceTransportPort, 2).unwrap(),
        FieldSpecifier::new(IE::destinationTransportPort, 2).unwrap(),
        FieldSpecifier::new(IE::protocolIdentifier, 1).unwrap(),
        FieldSpecifier::new(IE::tcpControlBits, 1).unwrap(),
        FieldSpecifier::new(IE::ipVersion, 1).unwrap(),
    ];

    let mut templates_map = HashMap::from([(1024, (vec![], fields.clone()))]);

    let good = NetFlowV9Packet::new(
        458441,
        Utc.with_ymd_and_hms(2017, 7, 25, 12, 50, 1).unwrap(),
        1,
        0,
        vec![Set::Data {
            id: DataSetId::new(1024).unwrap(),
            records: vec![
                DataRecord::new(
                    vec![],
                    vec![
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(
                            192, 168, 1, 100,
                        ))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            216, 58, 211, 99,
                        ))),
                        Field::flowEndSysUpTime(flowEndSysUpTime(107173)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(106988)),
                        Field::octetDeltaCount(octetDeltaCount(66)),
                        Field::packetDeltaCount(packetDeltaCount(1)),
                        Field::ingressInterface(ingressInterface(0)),
                        Field::egressInterface(egressInterface(0)),
                        Field::sourceTransportPort(sourceTransportPort(52357)),
                        Field::destinationTransportPort(destinationTransportPort(443)),
                        Field::protocolIdentifier(protocolIdentifier::UDP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, false, false, false, false, false, false, false,
                        )),
                        Field::ipVersion(ipVersion(4)),
                    ],
                ),
                DataRecord::new(
                    vec![],
                    vec![
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(
                            216, 58, 211, 99,
                        ))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            192, 168, 1, 100,
                        ))),
                        Field::flowEndSysUpTime(flowEndSysUpTime(107173)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(106988)),
                        Field::octetDeltaCount(octetDeltaCount(1378)),
                        Field::packetDeltaCount(packetDeltaCount(1)),
                        Field::ingressInterface(ingressInterface(0)),
                        Field::egressInterface(egressInterface(0)),
                        Field::sourceTransportPort(sourceTransportPort(443)),
                        Field::destinationTransportPort(destinationTransportPort(52357)),
                        Field::protocolIdentifier(protocolIdentifier::UDP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, false, false, false, false, false, false, false,
                        )),
                        Field::ipVersion(ipVersion(4)),
                    ],
                ),
                DataRecord::new(
                    vec![],
                    vec![
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(
                            192, 168, 1, 100,
                        ))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            216, 58, 211, 110,
                        ))),
                        Field::flowEndSysUpTime(flowEndSysUpTime(117589)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(117589)),
                        Field::octetDeltaCount(octetDeltaCount(66)),
                        Field::packetDeltaCount(packetDeltaCount(1)),
                        Field::ingressInterface(ingressInterface(0)),
                        Field::egressInterface(egressInterface(0)),
                        Field::sourceTransportPort(sourceTransportPort(63111)),
                        Field::destinationTransportPort(destinationTransportPort(443)),
                        Field::protocolIdentifier(protocolIdentifier::UDP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, false, false, false, false, false, false, false,
                        )),
                        Field::ipVersion(ipVersion(4)),
                    ],
                ),
                DataRecord::new(
                    vec![],
                    vec![
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(
                            192, 168, 1, 100,
                        ))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            216, 58, 211, 110,
                        ))),
                        Field::flowEndSysUpTime(flowEndSysUpTime(145525)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(145525)),
                        Field::octetDeltaCount(octetDeltaCount(51)),
                        Field::packetDeltaCount(packetDeltaCount(1)),
                        Field::ingressInterface(ingressInterface(0)),
                        Field::egressInterface(egressInterface(0)),
                        Field::sourceTransportPort(sourceTransportPort(63273)),
                        Field::destinationTransportPort(destinationTransportPort(443)),
                        Field::protocolIdentifier(protocolIdentifier::UDP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, false, false, false, false, false, false, false,
                        )),
                        Field::ipVersion(ipVersion(4)),
                    ],
                ),
            ],
        }],
    );

    test_parsed_completely_with_one_input(&good_wire, &mut templates_map, &good);
    test_write_with_two_inputs(&good, Some(&templates_map), true, &good_wire)?;
    Ok(())
}

#[test]
fn test_data_packet() -> Result<(), NetFlowV9WritingError> {
    let good_wire = [
        0x00, 0x09, 0x00, 0x02, 0x0c, 0x0a, 0x0b, 0x0e, 0x63, 0xd5, 0x45, 0x99, 0x00, 0xe3, 0xbd,
        0x93, 0x00, 0x00, 0x08, 0x21, 0x01, 0x39, 0x00, 0xec, 0x05, 0xde, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xcf, 0x00, 0x00, 0x00, 0xa1, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x02, 0x0c, 0x09,
        0xce, 0xb5, 0x0c, 0x09, 0xca, 0xc2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xd5, 0x03, 0xc4, 0x22, 0x8a, 0xbb, 0x6f, 0x74, 0x97, 0x3e,
        0x00, 0x35, 0x00, 0x00, 0x40, 0x00, 0x00, 0x06, 0x02, 0x00, 0x01, 0x60, 0x00, 0x00, 0x00,
        0x60, 0x00, 0x00, 0x05, 0x05, 0xde, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0x00, 0x00, 0x00, 0xa1,
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x02, 0x0c, 0x09, 0xce, 0xb5, 0x0c, 0x09, 0xca,
        0xc3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xd5, 0x03, 0xc4, 0x22, 0x8a, 0xbb, 0x6f, 0x74, 0x97, 0x42, 0x00, 0x35, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x06, 0x02, 0x00, 0x01, 0x60, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x05, 0x00,
        0x00,
    ];

    let field_specifiers = vec![
        FieldSpecifier::new(IE::mplsTopLabelStackSection, 3).unwrap(),
        FieldSpecifier::new(IE::mplsLabelStackSection2, 3).unwrap(),
        FieldSpecifier::new(IE::mplsLabelStackSection3, 3).unwrap(),
        FieldSpecifier::new(IE::mplsLabelStackSection4, 3).unwrap(),
        FieldSpecifier::new(IE::mplsLabelStackSection5, 3).unwrap(),
        FieldSpecifier::new(IE::mplsLabelStackSection6, 3).unwrap(),
        FieldSpecifier::new(IE::ingressInterface, 4).unwrap(),
        FieldSpecifier::new(IE::egressInterface, 4).unwrap(),
        FieldSpecifier::new(IE::octetDeltaCount, 4).unwrap(),
        FieldSpecifier::new(IE::packetDeltaCount, 4).unwrap(),
        FieldSpecifier::new(IE::flowEndSysUpTime, 4).unwrap(),
        FieldSpecifier::new(IE::flowStartSysUpTime, 4).unwrap(),
        FieldSpecifier::new(IE::mplsTopLabelIPv4Address, 4).unwrap(),
        FieldSpecifier::new(IE::sourceIPv6Address, 16).unwrap(),
        FieldSpecifier::new(IE::destinationIPv6Address, 16).unwrap(),
        FieldSpecifier::new(IE::flowLabelIPv6, 4).unwrap(),
        FieldSpecifier::new(IE::ipv6ExtensionHeaders, 4).unwrap(),
        FieldSpecifier::new(IE::sourceIPv4Address, 4).unwrap(),
        FieldSpecifier::new(IE::destinationIPv4Address, 4).unwrap(),
        FieldSpecifier::new(IE::sourceTransportPort, 2).unwrap(),
        FieldSpecifier::new(IE::destinationTransportPort, 2).unwrap(),
        FieldSpecifier::new(IE::mplsTopLabelPrefixLength, 1).unwrap(),
        FieldSpecifier::new(IE::mplsTopLabelType, 1).unwrap(),
        FieldSpecifier::new(IE::forwardingStatus, 1).unwrap(),
        FieldSpecifier::new(IE::flowDirection, 1).unwrap(),
        FieldSpecifier::new(IE::ipClassOfService, 1).unwrap(),
        FieldSpecifier::new(IE::protocolIdentifier, 1).unwrap(),
        FieldSpecifier::new(IE::tcpControlBits, 1).unwrap(),
        FieldSpecifier::new(IE::samplerId, 2).unwrap(),
        FieldSpecifier::new(IE::ingressVRFID, 4).unwrap(),
        FieldSpecifier::new(IE::egressVRFID, 4).unwrap(),
    ];

    let fields = (vec![], field_specifiers.clone());
    let mut templates_map = HashMap::from([(313, fields)]);

    let good = NetFlowV9Packet::new(
        201984782,
        Utc.with_ymd_and_hms(2023, 1, 28, 15, 56, 9).unwrap(),
        14925203,
        2081,
        vec![Set::Data {
            id: DataSetId::new(313).unwrap(),
            records: vec![
                DataRecord::new(
                    vec![],
                    vec![
                        Field::mplsTopLabelStackSection(mplsTopLabelStackSection(vec![
                            0x05, 0xde, 0x01,
                        ])),
                        Field::mplsLabelStackSection2(mplsLabelStackSection2(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection3(mplsLabelStackSection3(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection4(mplsLabelStackSection4(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection5(mplsLabelStackSection5(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection6(mplsLabelStackSection6(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::ingressInterface(ingressInterface(207)),
                        Field::egressInterface(egressInterface(161)),
                        Field::octetDeltaCount(octetDeltaCount(128)),
                        Field::packetDeltaCount(packetDeltaCount(2)),
                        Field::flowEndSysUpTime(flowEndSysUpTime(0x0c09ceb5)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(0x0c09cac2)),
                        Field::mplsTopLabelIPv4Address(mplsTopLabelIPv4Address(Ipv4Addr::new(
                            0, 0, 0, 0,
                        ))),
                        Field::sourceIPv6Address(sourceIPv6Address(Ipv6Addr::from(0))),
                        Field::destinationIPv6Address(destinationIPv6Address(Ipv6Addr::from(0))),
                        Field::flowLabelIPv6(flowLabelIPv6(0)),
                        Field::ipv6ExtensionHeaders(ipv6ExtensionHeaders(0)),
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(213, 3, 196, 34))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            138, 187, 111, 116,
                        ))),
                        Field::sourceTransportPort(sourceTransportPort(38718)),
                        Field::destinationTransportPort(destinationTransportPort(53)),
                        Field::mplsTopLabelPrefixLength(mplsTopLabelPrefixLength(0)),
                        Field::mplsTopLabelType(mplsTopLabelType::Unknown),
                        Field::forwardingStatus(forwardingStatus::Forwarded(
                            ie::forwardingStatusForwardedReason::Unknown,
                        )),
                        Field::flowDirection(flowDirection::ingress),
                        Field::ipClassOfService(ipClassOfService(0)),
                        Field::protocolIdentifier(protocolIdentifier::TCP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, true, false, false, false, false, false, false,
                        )),
                        Field::samplerId(samplerId(1)),
                        Field::ingressVRFID(ingressVRFID(1610612736)),
                        Field::egressVRFID(egressVRFID(1610612741)),
                    ],
                ),
                DataRecord::new(
                    vec![],
                    vec![
                        Field::mplsTopLabelStackSection(mplsTopLabelStackSection(vec![
                            0x05, 0xde, 0x01,
                        ])),
                        Field::mplsLabelStackSection2(mplsLabelStackSection2(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection3(mplsLabelStackSection3(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection4(mplsLabelStackSection4(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection5(mplsLabelStackSection5(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::mplsLabelStackSection6(mplsLabelStackSection6(vec![
                            0x00, 0x00, 0x00,
                        ])),
                        Field::ingressInterface(ingressInterface(207)),
                        Field::egressInterface(egressInterface(161)),
                        Field::octetDeltaCount(octetDeltaCount(128)),
                        Field::packetDeltaCount(packetDeltaCount(2)),
                        Field::flowEndSysUpTime(flowEndSysUpTime(0x0c09ceb5)),
                        Field::flowStartSysUpTime(flowStartSysUpTime(0x0c09cac3)),
                        Field::mplsTopLabelIPv4Address(mplsTopLabelIPv4Address(Ipv4Addr::new(
                            0, 0, 0, 0,
                        ))),
                        Field::sourceIPv6Address(sourceIPv6Address(Ipv6Addr::from(0))),
                        Field::destinationIPv6Address(destinationIPv6Address(Ipv6Addr::from(0))),
                        Field::flowLabelIPv6(flowLabelIPv6(0)),
                        Field::ipv6ExtensionHeaders(ipv6ExtensionHeaders(0)),
                        Field::sourceIPv4Address(sourceIPv4Address(Ipv4Addr::new(213, 3, 196, 34))),
                        Field::destinationIPv4Address(destinationIPv4Address(Ipv4Addr::new(
                            138, 187, 111, 116,
                        ))),
                        Field::sourceTransportPort(sourceTransportPort(38722)),
                        Field::destinationTransportPort(destinationTransportPort(53)),
                        Field::mplsTopLabelPrefixLength(mplsTopLabelPrefixLength(0)),
                        Field::mplsTopLabelType(mplsTopLabelType::Unknown),
                        Field::forwardingStatus(forwardingStatus::Forwarded(
                            ie::forwardingStatusForwardedReason::Unknown,
                        )),
                        Field::flowDirection(flowDirection::ingress),
                        Field::ipClassOfService(ipClassOfService(0)),
                        Field::protocolIdentifier(protocolIdentifier::TCP),
                        Field::tcpControlBits(TCPHeaderFlags::new(
                            false, true, false, false, false, false, false, false,
                        )),
                        Field::samplerId(samplerId(1)),
                        Field::ingressVRFID(ingressVRFID(1610612736)),
                        Field::egressVRFID(egressVRFID(1610612741)),
                    ],
                ),
            ],
        }],
    );

    test_parsed_completely_with_one_input(&good_wire, &mut templates_map, &good);
    test_write_with_two_inputs(&good, Some(&templates_map), true, &good_wire)?;
    Ok(())
}

#[test]
fn test_mix_option_template_set() -> Result<(), SetWritingError> {
    let good_wire = [
        0x00, 0x01, 0x00, 0x1a, 0x01, 0x15, 0x00, 0x04, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x0a, 0x00, 0x02, 0x00, 0x52, 0x00, 0x10, 0x00, 0x53, 0x00, 0x20,
    ];
    let good = Set::OptionsTemplate(vec![OptionsTemplateRecord::new(
        277,
        vec![ScopeFieldSpecifier::new(ScopeIE::System, 4)],
        vec![
            FieldSpecifier::new(IE::ingressInterface, 2).unwrap(),
            FieldSpecifier::new(IE::interfaceName, 16).unwrap(),
            FieldSpecifier::new(IE::interfaceDescription, 32).unwrap(),
        ],
    )]);

    let mut templates_map = HashMap::new();
    test_parsed_completely_with_one_input(&good_wire, &mut templates_map, &good);
    test_write_with_two_inputs(&good, Some(&templates_map), false, &good_wire)?;
    Ok(())
}

#[test]
fn test_mix_option_template_set2() -> Result<(), SetWritingError> {
    let good_wire = [
        0x00, 0x01, 0x00, 0x18, 0x01, 0x4e, 0x00, 0x04, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, 0x00,
        0xea, 0x00, 0x04, 0x00, 0xec, 0x00, 0x20, 0x00, 0x00,
    ];
    let good = Set::OptionsTemplate(vec![OptionsTemplateRecord::new(
        334,
        vec![ScopeFieldSpecifier::new(ScopeIE::System, 4)],
        vec![
            FieldSpecifier::new(IE::ingressVRFID, 4).unwrap(),
            FieldSpecifier::new(IE::VRFname, 32).unwrap(),
        ],
    )]);

    let mut templates_map = HashMap::new();
    test_parsed_completely_with_one_input(&good_wire, &mut templates_map, &good);
    test_write_with_two_inputs(&good, Some(&templates_map), true, &good_wire)?;
    Ok(())
}

#[test]
fn test_padding() -> Result<(), NetFlowV9WritingError> {
    let good_no_padding_wire = [
        0x00, 0x09, // Version
        0x00, 0x02, // Count
        0x0f, 0x5e, 0x5c, 0x6b, // Sys up time
        0x63, 0xd5, 0x45, 0x85, // Timestamp
        0x00, 0x09, 0x43, 0x2a, // seq
        0x00, 0x00, 0x00, 0x06, // Source Id
        0x00, 0x01, // Options Template
        0x00, 0x1e, // Length
        0x01, 0x02, // Options template ID
        0x00, 0x04, // Scope Length
        0x00, 0x10, // Options Length
        0x00, 0x01, 0x00, 0x04, 0x00, 0x30, 0x00, 0x04, 0x00, 0x54, 0x00, 0x28, 0x00, 0x31, 0x00,
        0x01, 0x00, 0x32, 0x00, 0x02, 0x01, 0x02, // Flow Set ID
        0x00, 0x37, // Flow Set Length
        0xd5, 0x03, 0xdf, 0x23, // Scope System
        0x00, 0x00, 0x00, 0x02, // Sampler ID
        0x4e, 0x45, 0x54, 0x46, // Sampler Name
        0x4c, 0x4f, 0x57, 0x2d, // Sampler Name
        0x53, 0x41, 0x4d, 0x50, // Sampler Name
        0x4c, 0x45, 0x52, 0x2d, // Sampler Name
        0x4d, 0x41, 0x50, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x02, // Random
        0x01, 0x00, // Sampler Random Interval
    ];

    let good_with_padding_wire = [
        0x00, 0x09, // Version
        0x00, 0x02, // Count
        0x0f, 0x5e, 0x5c, 0x6b, // Sys up time
        0x63, 0xd5, 0x45, 0x85, // Timestamp
        0x00, 0x09, 0x43, 0x2a, // seq
        0x00, 0x00, 0x00, 0x06, // Source Id
        0x00, 0x01, // Options Template
        0x00, 0x20, // Length
        0x01, 0x02, // Options template ID
        0x00, 0x04, // Scope Length
        0x00, 0x10, // Options Length
        0x00, 0x01, 0x00, 0x04, // Scope
        0x00, 0x30, 0x00, 0x04, // Field
        0x00, 0x54, 0x00, 0x28, // Field
        0x00, 0x31, 0x00, 0x01, // Field
        0x00, 0x32, 0x00, 0x02, 0x00, 0x00, // Padding
        0x01, 0x02, // Flow Set ID
        0x00, 0x3a, // Flow Set Length
        0xd5, 0x03, 0xdf, 0x23, // Scope System
        0x00, 0x00, 0x00, 0x02, // Sampler ID
        0x4e, 0x45, 0x54, 0x46, // Sampler Name
        0x4c, 0x4f, 0x57, 0x2d, // Sampler Name
        0x53, 0x41, 0x4d, 0x50, // Sampler Name
        0x4c, 0x45, 0x52, 0x2d, // Sampler Name
        0x4d, 0x41, 0x50, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x02, // Random
        0x01, 0x00, // Sampler Random Interval
        0x00, 0x00, 0x00, // Padding
    ];

    let bad_padding_options_wire = [
        0x00, 0x09, // Version
        0x00, 0x02, // Count
        0x0f, 0x5e, 0x5c, 0x6b, // Sys up time
        0x63, 0xd5, 0x45, 0x85, // Timestamp
        0x00, 0x09, 0x43, 0x2a, // seq
        0x00, 0x00, 0x00, 0x06, // Source Id
        0x00, 0x01, // Options Template
        0x00, 0x20, // Length
        0x01, 0x02, // Options template ID
        0x00, 0x04, // Scope Length
        0x00, 0x10, // Options Length
        0x00, 0x01, 0x00, 0x04, // Scope
        0x00, 0x30, 0x00, 0x04, // Field
        0x00, 0x54, 0x00, 0x28, // Field
        0x00, 0x31, 0x00, 0x01, // Field
        0x00, 0x32, 0x00, 0x02, 0x00, 0x11, // Padding
        0x01, 0x02, // Flow Set ID
        0x00, 0x3a, // Flow Set Length
        0xd5, 0x03, 0xdf, 0x23, // Scope System
        0x00, 0x00, 0x00, 0x02, // Sampler ID
        0x4e, 0x45, 0x54, 0x46, // Sampler Name
        0x4c, 0x4f, 0x57, 0x2d, // Sampler Name
        0x53, 0x41, 0x4d, 0x50, // Sampler Name
        0x4c, 0x45, 0x52, 0x2d, // Sampler Name
        0x4d, 0x41, 0x50, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x02, // Random
        0x01, 0x00, // Sampler Random Interval
        0x00, 0x00, 0x00, // Padding
    ];

    let bad_padding_data_wire = [
        0x00, 0x09, // Version
        0x00, 0x02, // Count
        0x0f, 0x5e, 0x5c, 0x6b, // Sys up time
        0x63, 0xd5, 0x45, 0x85, // Timestamp
        0x00, 0x09, 0x43, 0x2a, // seq
        0x00, 0x00, 0x00, 0x06, // Source Id
        0x00, 0x01, // Options Template
        0x00, 0x20, // Length
        0x01, 0x02, // Options template ID
        0x00, 0x04, // Scope Length
        0x00, 0x10, // Options Length
        0x00, 0x01, 0x00, 0x04, // Scope
        0x00, 0x30, 0x00, 0x04, // Field
        0x00, 0x54, 0x00, 0x28, // Field
        0x00, 0x31, 0x00, 0x01, // Field
        0x00, 0x32, 0x00, 0x02, 0x00, 0x00, // Padding
        0x01, 0x02, // Flow Set ID
        0x00, 0x3a, // Flow Set Length
        0xd5, 0x03, 0xdf, 0x23, // Scope System
        0x00, 0x00, 0x00, 0x02, // Sampler ID
        0x4e, 0x45, 0x54, 0x46, // Sampler Name
        0x4c, 0x4f, 0x57, 0x2d, // Sampler Name
        0x53, 0x41, 0x4d, 0x50, // Sampler Name
        0x4c, 0x45, 0x52, 0x2d, // Sampler Name
        0x4d, 0x41, 0x50, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x00, 0x00, 0x00, 0x00, // Sampler Name
        0x02, // Random
        0x01, 0x00, // Sampler Random Interval
        0x01, 0x00, 0x00, // Padding
    ];

    let bad_options_template_padding = LocatedNetFlowV9PacketParsingError::new(
        unsafe { Span::new_from_raw_offset(51, &[17]) },
        NetFlowV9PacketParsingError::SetError(SetParsingError::InvalidPaddingValue(17)),
    );

    let bad_data_padding = LocatedNetFlowV9PacketParsingError::new(
        unsafe { Span::new_from_raw_offset(107, &[0x01, 0x00, 0x00]) },
        NetFlowV9PacketParsingError::SetError(SetParsingError::InvalidPaddingValue(1)),
    );
    let mut templates_no_padding_map = HashMap::new();
    let mut templates_with_padding_map = HashMap::new();
    let mut template_bad_map = HashMap::new();

    let (_, good_no_padding) = NetFlowV9Packet::from_wire(
        Span::new(&good_no_padding_wire),
        &mut templates_no_padding_map,
    )
    .unwrap();
    let (_, good_with_padding) = NetFlowV9Packet::from_wire(
        Span::new(&good_with_padding_wire),
        &mut templates_with_padding_map,
    )
    .unwrap();

    test_parse_error_with_one_input::<
        NetFlowV9Packet,
        &mut TemplatesMap,
        LocatedNetFlowV9PacketParsingError<'_>,
    >(
        &bad_padding_options_wire,
        &mut template_bad_map,
        &bad_options_template_padding,
    );
    test_parse_error_with_one_input::<
        NetFlowV9Packet,
        &mut TemplatesMap,
        LocatedNetFlowV9PacketParsingError<'_>,
    >(
        &bad_padding_data_wire,
        &mut template_bad_map,
        &bad_data_padding,
    );

    // Packets should be equal regardless of the padding
    test_parsed_completely_with_one_input(
        &good_no_padding_wire,
        &mut templates_no_padding_map,
        &good_with_padding,
    );
    test_parsed_completely_with_one_input(
        &good_with_padding_wire,
        &mut templates_with_padding_map,
        &good_no_padding,
    );

    test_write_with_two_inputs(
        &good_no_padding,
        Some(&templates_no_padding_map),
        false,
        &good_no_padding_wire,
    )?;
    test_write_with_two_inputs(
        &good_with_padding,
        Some(&templates_with_padding_map),
        true,
        &good_with_padding_wire,
    )?;
    Ok(())
}

#[test]
fn test_with_iana_subregs() -> Result<(), NetFlowV9WritingError> {
    let good_template_wire = [
        0x00, 0x09, 0x00, 0x01, 0x00, 0x00, 0x00, 0x78, 0x66, 0x8b, 0xe2, 0xd0, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6c, 0x01, 0x90, 0x00, 0x19, 0x00, 0x08,
        0x00, 0x04, 0x00, 0x0c, 0x00, 0x04, 0x00, 0x07, 0x00, 0x02, 0x00, 0x0b, 0x00, 0x02, 0x00,
        0x94, 0x00, 0x08, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x04,
        0x00, 0x2e, 0x00, 0x01, 0x00, 0x59, 0x00, 0x04, 0x00, 0x65, 0x00, 0x01, 0x00, 0x88, 0x00,
        0x01, 0x00, 0xe5, 0x00, 0x01, 0x00, 0xe9, 0x00, 0x01, 0x00, 0xef, 0x00, 0x01, 0x01, 0x15,
        0x00, 0x01, 0x01, 0x1e, 0x00, 0x02, 0x01, 0x29, 0x00, 0x01, 0x01, 0x80, 0x00, 0x01, 0x01,
        0x86, 0x00, 0x02, 0x01, 0x98, 0x00, 0x02, 0x01, 0xc0, 0x00, 0x01, 0x01, 0xd2, 0x00, 0x01,
        0x01, 0xd3, 0x00, 0x01, 0x01, 0xf4, 0x00, 0x01,
    ];

    let good_data_wire = [
        0x00, 0x09, 0x00, 0x01, 0x00, 0x00, 0x00, 0x78, 0x66, 0x8b, 0xe2, 0xd0, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x90, 0x00, 0x38, 0x0a, 0x64, 0x00, 0x01, 0x0a, 0x64,
        0x00, 0x97, 0x27, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9a, 0x21, 0x12, 0x01,
        0x00, 0x00, 0x04, 0xb0, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x86, 0x12, 0x05,
        0x0f, 0x02, 0x03, 0x01, 0x00, 0x06, 0x05, 0x04, 0x00, 0x04, 0x00, 0x0a, 0x04, 0x04, 0x01,
        0x05,
    ];

    let good_template = NetFlowV9Packet::new(
        120,
        Utc.with_ymd_and_hms(2024, 7, 8, 13, 0, 0).unwrap(),
        0,
        0,
        vec![Set::Template(vec![TemplateRecord::new(
            400,
            vec![
                FieldSpecifier::new(ie::IE::sourceIPv4Address, 4).unwrap(),
                FieldSpecifier::new(ie::IE::destinationIPv4Address, 4).unwrap(),
                FieldSpecifier::new(ie::IE::sourceTransportPort, 2).unwrap(),
                FieldSpecifier::new(ie::IE::destinationTransportPort, 2).unwrap(),
                FieldSpecifier::new(ie::IE::flowId, 8).unwrap(),
                FieldSpecifier::new(ie::IE::protocolIdentifier, 1).unwrap(),
                FieldSpecifier::new(ie::IE::octetDeltaCount, 4).unwrap(),
                FieldSpecifier::new(ie::IE::packetDeltaCount, 4).unwrap(),
                FieldSpecifier::new(ie::IE::mplsTopLabelType, 1).unwrap(),
                FieldSpecifier::new(ie::IE::forwardingStatus, 4).unwrap(),
                FieldSpecifier::new(ie::IE::classificationEngineId, 1).unwrap(),
                FieldSpecifier::new(ie::IE::flowEndReason, 1).unwrap(),
                FieldSpecifier::new(ie::IE::natOriginatingAddressRealm, 1).unwrap(),
                FieldSpecifier::new(ie::IE::firewallEvent, 1).unwrap(),
                FieldSpecifier::new(ie::IE::biflowDirection, 1).unwrap(),
                FieldSpecifier::new(ie::IE::observationPointType, 1).unwrap(),
                FieldSpecifier::new(ie::IE::anonymizationTechnique, 2).unwrap(),
                FieldSpecifier::new(ie::IE::natType, 1).unwrap(),
                FieldSpecifier::new(ie::IE::valueDistributionMethod, 1).unwrap(),
                FieldSpecifier::new(ie::IE::flowSelectorAlgorithm, 2).unwrap(),
                FieldSpecifier::new(ie::IE::dataLinkFrameType, 2).unwrap(),
                FieldSpecifier::new(ie::IE::mibCaptureTimeSemantics, 1).unwrap(),
                FieldSpecifier::new(ie::IE::natQuotaExceededEvent, 1).unwrap(),
                FieldSpecifier::new(ie::IE::natThresholdEvent, 1).unwrap(),
                FieldSpecifier::new(ie::IE::srhIPv6ActiveSegmentType, 1).unwrap(),
            ],
        )])],
    );

    let good_data = NetFlowV9Packet::new(
        120,
        Utc.with_ymd_and_hms(2024, 7, 8, 13, 0, 0).unwrap(),
        1,
        0,
        vec![Set::Data {
            id: DataSetId::new(400).unwrap(),
            records: vec![DataRecord::new(
                vec![],
                vec![
                    ie::Field::sourceIPv4Address(ie::sourceIPv4Address(Ipv4Addr::new(
                        10, 100, 0, 1,
                    ))),
                    ie::Field::destinationIPv4Address(ie::destinationIPv4Address(Ipv4Addr::new(
                        10, 100, 0, 151,
                    ))),
                    ie::Field::sourceTransportPort(ie::sourceTransportPort(10004)),
                    ie::Field::destinationTransportPort(ie::destinationTransportPort(1)),
                    ie::Field::flowId(ie::flowId(10101010)),
                    ie::Field::protocolIdentifier(ie::protocolIdentifier::ICMP),
                    ie::Field::octetDeltaCount(ie::octetDeltaCount(1200)),
                    ie::Field::packetDeltaCount(ie::packetDeltaCount(1)),
                    ie::Field::mplsTopLabelType(ie::mplsTopLabelType::Unknown),
                    ie::Field::forwardingStatus(ie::forwardingStatus::Dropped(
                        ie::forwardingStatusDroppedReason::Badheaderchecksum,
                    )),
                    ie::Field::classificationEngineId(ie::classificationEngineId::ETHERTYPE),
                    ie::Field::flowEndReason(ie::flowEndReason::lackofresources),
                    ie::Field::natOriginatingAddressRealm(
                        ie::natOriginatingAddressRealm::Unassigned(15),
                    ),
                    ie::Field::firewallEvent(ie::firewallEvent::FlowDeleted),
                    ie::Field::biflowDirection(ie::biflowDirection::perimeter),
                    ie::Field::observationPointType(ie::observationPointType::Physicalport),
                    ie::Field::anonymizationTechnique(
                        ie::anonymizationTechnique::StructuredPermutation,
                    ),
                    ie::Field::natType(ie::natType::NAT66translated),
                    ie::Field::valueDistributionMethod(
                        ie::valueDistributionMethod::SimpleUniformDistribution,
                    ),
                    ie::Field::flowSelectorAlgorithm(
                        ie::flowSelectorAlgorithm::UniformprobabilisticSampling,
                    ),
                    ie::Field::dataLinkFrameType(ie::dataLinkFrameType::Unassigned(10)),
                    ie::Field::mibCaptureTimeSemantics(ie::mibCaptureTimeSemantics::average),
                    ie::Field::natQuotaExceededEvent(
                        ie::natQuotaExceededEvent::Maximumactivehostsorsubscribers,
                    ),
                    ie::Field::natThresholdEvent(
                        ie::natThresholdEvent::Addresspoolhighthresholdevent,
                    ),
                    ie::Field::srhIPv6ActiveSegmentType(
                        ie::srhIPv6ActiveSegmentType::BGPSegmentRoutingPrefixSID,
                    ),
                ],
            )],
        }],
    );

    let mut templates_map = HashMap::new();
    test_parsed_completely_with_one_input(&good_template_wire, &mut templates_map, &good_template);
    test_parsed_completely_with_one_input(&good_data_wire, &mut templates_map, &good_data);

    test_write_with_one_input(&good_template, Some(&templates_map), &good_template_wire)?;
    test_write_with_one_input(&good_data, Some(&templates_map), &good_data_wire)?;

    Ok(())
}

#[test]
fn test_zero_length_fields() {
    let good_template_wire = [
        0, 9, 75, 9, 0, 0, 96, 0, 33, 0, 0, 0, 47, 0, 9, 1, 0, 0, 0, 0, 0, 1, 0, 15, 91, 0, 0, 4,
        0, 0, 91, 0, 0, 0, 0, 91, 0, 0, 4, 0, 0, 0, 0, 32, 0, 0,
    ];
    let mut templates_map = HashMap::new();
    // The test here will produce invalid packet, but what we are testing for is not
    // crashing due to divide by zero
    let ret = NetFlowV9Packet::from_wire(Span::new(&good_template_wire), &mut templates_map);
    assert!(ret.is_err());
}

#[test]
fn test_records_len_larger_than_count() {
    let good_template_wire = [
        0, 9, 0, 64, 16, 1, 0, 0, 42, 0, 0, 64, 16, 0, 53, 255, 255, 1, 1, 0, 0, 1, 0, 150, 158, 0,
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 171, 0, 0, 8, 0, 0, 0,
        0, 0, 8, 0, 122, 0, 148, 251, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 171, 0,
        0, 8, 0, 0, 0, 0, 0, 8, 64, 0, 0, 1, 251, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
        0, 171, 0, 0, 8, 0, 0, 0, 176, 251, 0, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        171, 0, 0, 24, 0, 0, 1, 0, 0, 0, 0, 0, 0, 4, 251, 0, 0, 0, 0, 9, 9, 255, 255, 0, 0, 0, 6,
        0, 0, 0, 0, 0, 171, 0, 1, 8, 0, 0, 0, 0, 171, 0, 0, 123, 255, 0, 0, 123, 123, 123, 255, 0,
        0, 0, 0, 0, 0, 0, 171, 0, 0, 24, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 123, 123, 123, 123, 123,
        255, 0, 41, 212, 251, 0, 0, 0, 0, 0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123, 255, 0, 0, 0,
        0, 0, 0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123, 255, 0, 0, 0, 41, 212, 251, 0, 0, 0, 0,
        0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123, 255, 0, 41, 212, 251, 0, 0, 0, 0, 0, 0, 50, 0,
        6, 0, 123, 123, 123, 123, 123, 255, 0, 0, 123, 123, 123, 255, 0, 0, 0, 0, 0, 0, 0, 123,
        123, 123, 255, 0, 41, 212, 251, 0, 0, 0, 0, 0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123,
        255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 41, 212, 0, 123, 123, 123, 123, 123, 255, 0, 41,
        212, 251, 0, 0, 0, 0, 0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123, 255, 0, 0, 123, 0, 0, 0,
        0, 0, 50, 0, 6, 0, 123, 123, 123, 123, 123, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 41,
        212, 0, 123, 123, 123, 123, 123, 255, 0, 41, 212, 251, 0, 0, 0, 0, 0, 0, 50, 0, 6, 0, 123,
        123, 123, 123, 123, 255, 0, 0,
    ];
    let mut templates_map = HashMap::new();
    // The test here will produce invalid packet, but what we are testing for is not
    // crashing due subtracting count of records from the templates
    let ret = NetFlowV9Packet::from_wire(Span::new(&good_template_wire), &mut templates_map);
    assert!(ret.is_err());
}
