use crate::emit;
use metrics::counter;
use vector_core::internal_event::InternalEvent;

use vector_common::internal_event::{
    error_stage, error_type, ComponentEventsDropped, UNINTENTIONAL,
};

#[derive(Debug)]
pub struct DecoderFramingError<E> {
    pub error: E,
}

impl<E: std::fmt::Display> InternalEvent for DecoderFramingError<E> {
    fn emit(self) {
        error!(
            message = "Failed framing bytes.",
            error = %self.error,
            error_code = "decoder_frame",
            error_type = error_type::PARSER_FAILED,
            stage = error_stage::PROCESSING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_code" => "decoder_frame",
            "error_type" => error_type::PARSER_FAILED,
            "stage" => error_stage::PROCESSING,
        );
    }
}

#[derive(Debug)]
pub struct DecoderDeserializeError<'a> {
    pub error: &'a crate::Error,
}

impl<'a> InternalEvent for DecoderDeserializeError<'a> {
    fn emit(self) {
        error!(
            message = "Failed deserializing frame.",
            error = %self.error,
            error_code = "decoder_deserialize",
            error_type = error_type::PARSER_FAILED,
            stage = error_stage::PROCESSING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_code" => "decoder_deserialize",
            "error_type" => error_type::PARSER_FAILED,
            "stage" => error_stage::PROCESSING,
        );
    }
}

#[derive(Debug)]
pub struct EncoderFramingError<'a> {
    pub error: &'a codecs::encoding::BoxedFramingError,
}

impl<'a> InternalEvent for EncoderFramingError<'a> {
    fn emit(self) {
        let reason = "Failed framing bytes.";
        error!(
            message = reason,
            error = %self.error,
            error_code = "encoder_frame",
            error_type = error_type::ENCODER_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_code" => "encoder_frame",
            "error_type" => error_type::ENCODER_FAILED,
            "stage" => error_stage::SENDING,
        );
        emit!(ComponentEventsDropped::<UNINTENTIONAL> { count: 1, reason });
    }
}

#[derive(Debug)]
pub struct EncoderSerializeError<'a> {
    pub error: &'a crate::Error,
}

impl<'a> InternalEvent for EncoderSerializeError<'a> {
    fn emit(self) {
        let reason = "Failed serializing frame.";
        error!(
            message = reason,
            error = %self.error,
            error_code = "encoder_serialize",
            error_type = error_type::ENCODER_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_code" => "encoder_serialize",
            "error_type" => error_type::ENCODER_FAILED,
            "stage" => error_stage::SENDING,
        );
        emit!(ComponentEventsDropped::<UNINTENTIONAL> { count: 1, reason });
    }
}

#[derive(Debug)]
pub struct EncoderWriteError<'a, E> {
    pub error: &'a E,
    pub count: usize,
}

impl<E: std::fmt::Display> InternalEvent for EncoderWriteError<'_, E> {
    fn emit(self) {
        let reason = "Failed writing bytes.";
        error!(
            message = reason,
            error = %self.error,
            error_type = error_type::IO_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::ENCODER_FAILED,
            "stage" => error_stage::SENDING,
        );
        if self.count > 0 {
            emit!(ComponentEventsDropped::<UNINTENTIONAL> {
                count: self.count,
                reason,
            });
        }
    }
}
