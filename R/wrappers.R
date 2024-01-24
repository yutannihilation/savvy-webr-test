#' @useDynLib savvyExamples, .registration = TRUE
#' @keywords internal
NULL

#' @export
int_vec <- function() {
  .Call(int_vec__impl)
}


