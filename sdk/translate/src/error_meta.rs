// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Another modification is being made. That modification must complete before you can make
    /// your change.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>There was a conflict processing the request. Try your request again.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>The confidence that Amazon Comprehend accurately detected the source language is low. If a
    /// low confidence level is acceptable for your application, you can use the language in the
    /// exception to call Amazon Translate again. For more information, see the <a href="https://docs.aws.amazon.com/comprehend/latest/dg/API_DetectDominantLanguage.html">DetectDominantLanguage</a> operation in the <i>Amazon Comprehend Developer
    /// Guide</i>. </p>
    DetectedLanguageLowConfidenceException(crate::error::DetectedLanguageLowConfidenceException),
    /// <p>An internal server error occurred. Retry your request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The filter specified for the operation is invalid. Specify a different filter.</p>
    InvalidFilterException(crate::error::InvalidFilterException),
    /// <p>The value of the parameter is invalid. Review the value of the parameter you are using to
    /// correct it, and then retry your operation.</p>
    InvalidParameterValueException(crate::error::InvalidParameterValueException),
    /// <p> The request that you made is invalid. Check your request to determine why it's invalid
    /// and then retry the request. </p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified limit has been exceeded. Review your request and retry it with a quantity
    /// below the stated limit.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The resource you are looking for has not been found. Review the resource you're looking
    /// for and see if a different resource will accomplish your needs before retrying the revised
    /// request.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The Amazon Translate service is temporarily unavailable. Please wait a bit and then retry your
    /// request.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p> The size of the text you submitted exceeds the size limit. Reduce the size of the text or
    /// use a smaller document and then retry your request. </p>
    TextSizeLimitExceededException(crate::error::TextSizeLimitExceededException),
    /// <p> You have made too many requests within a short period of time. Wait for a short time and
    /// then try your request again.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>Amazon Translate does not support translation from the language of the source text into the requested
    /// target language. For more information, see <a>how-to-error-msg</a>. </p>
    UnsupportedLanguagePairException(crate::error::UnsupportedLanguagePairException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DetectedLanguageLowConfidenceException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidFilterException(inner) => inner.fmt(f),
            Error::InvalidParameterValueException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TextSizeLimitExceededException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::UnsupportedLanguagePairException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateParallelDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateParallelDataError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateParallelDataErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateParallelDataErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateParallelDataErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::CreateParallelDataErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::CreateParallelDataErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateParallelDataErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateParallelDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteParallelDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteParallelDataError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteParallelDataErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::DeleteParallelDataErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteParallelDataErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteParallelDataErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteParallelDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteTerminologyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteTerminologyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteTerminologyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteTerminologyErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::DeleteTerminologyErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteTerminologyErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteTerminologyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeTextTranslationJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeTextTranslationJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeTextTranslationJobErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::DescribeTextTranslationJobErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeTextTranslationJobErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::DescribeTextTranslationJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetParallelDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetParallelDataError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetParallelDataErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetParallelDataErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::GetParallelDataErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetParallelDataErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetParallelDataErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTerminologyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTerminologyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetTerminologyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetTerminologyErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::GetTerminologyErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetTerminologyErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetTerminologyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ImportTerminologyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ImportTerminologyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ImportTerminologyErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ImportTerminologyErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ImportTerminologyErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::ImportTerminologyErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ImportTerminologyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListParallelDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListParallelDataError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListParallelDataErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListParallelDataErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ListParallelDataErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListParallelDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTerminologiesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTerminologiesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTerminologiesErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTerminologiesErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ListTerminologiesErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListTerminologiesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTextTranslationJobsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTextTranslationJobsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTextTranslationJobsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListTextTranslationJobsErrorKind::InvalidFilterException(inner) => {
                    Error::InvalidFilterException(inner)
                }
                crate::error::ListTextTranslationJobsErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListTextTranslationJobsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListTextTranslationJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartTextTranslationJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartTextTranslationJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartTextTranslationJobErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::StartTextTranslationJobErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::StartTextTranslationJobErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::StartTextTranslationJobErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::StartTextTranslationJobErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::StartTextTranslationJobErrorKind::UnsupportedLanguagePairException(inner) => Error::UnsupportedLanguagePairException(inner),
                crate::error::StartTextTranslationJobErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopTextTranslationJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StopTextTranslationJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopTextTranslationJobErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StopTextTranslationJobErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopTextTranslationJobErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::StopTextTranslationJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TranslateTextError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TranslateTextError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TranslateTextErrorKind::DetectedLanguageLowConfidenceException(
                    inner,
                ) => Error::DetectedLanguageLowConfidenceException(inner),
                crate::error::TranslateTextErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::TranslateTextErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::TranslateTextErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TranslateTextErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::TranslateTextErrorKind::TextSizeLimitExceededException(inner) => {
                    Error::TextSizeLimitExceededException(inner)
                }
                crate::error::TranslateTextErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::TranslateTextErrorKind::UnsupportedLanguagePairException(inner) => {
                    Error::UnsupportedLanguagePairException(inner)
                }
                crate::error::TranslateTextErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateParallelDataError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateParallelDataError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateParallelDataErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::UpdateParallelDataErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::UpdateParallelDataErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateParallelDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
