% lab14_ex_speech_compress.m
% Speech compression using linear prediction coding
clear all; close all;
ifigs = 0;             %  0/1 - to show figures inside the processing loop? 

% Parameters
Mlen=240;              % length of analyzed block of samples
Mstep=180;             % offset between analyzed data blocks (in samples)
Np=2;                  % prediction order (IIR-AR filter order)
where=181;             % initial position of the first voiced excitation
roffset=20;            % offset in auto-correlation function when find max
compress=[];           % table for calculated speech model coefficients
s=[];                  % the whole synthesized speech
ss=[];                 % one fragment of synthesized speech

% Read signal to compress
[x,fs]=audioread('speech.wav');	   % read speech signal (audio/wav/read)
figure; plot(x); title('Speech');  % display it
soundsc(x,fs); pause               % play it on loudspeakers (headphones)
N=length(x);                       % signal length
bs=zeros(1,Np);			           % synthesis filter buffer
Nblocks=floor((N-Mlen)/Mstep+1);   % number of speech blocks to be analyzed

% MAIN PROCESSING LOOP
for  nr = 1 : Nblocks
   % take new data block (fragment of speech samples)
     n = 1+(nr-1)*Mstep : Mlen + (nr-1)*Mstep;
     bx = x(n);
   % ANALYSIS - calculate speech model parameters
	 bx = bx - mean(bx);                             % remove mean value
     for k = 0 : Mlen-1
         r(k+1)=sum( bx(1:Mlen-k) .* bx(1+k:Mlen) ); % calculate auto-correlation
     end                                             % try: r=xcorr(x,'unbiased')
     if(ifigs==1)
       subplot(411); plot(n,bx); title('Input speech x(n)');
       subplot(412); plot(r); title('Its auto-correlation rxx(k)');
     end
     [rmax,imax] = max( r(roffset : Mlen)  );        % find max of auto-correlation
     imax = imax+(roffset-1);                        % its argument (position)
     if ( rmax > 0.35*r(1) )  T=imax; else T=0; end  % is the speech periodic?
     if (T>80) T=round(T/2); end                     % second sub-harmonic found
     T, % pause                                      % display speech period T
     rr(1:Np,1)=(r(2:Np+1))';                    % create an auto-correlation vector 
     for m=1:Np                                  %
         R(m,1:Np)=[r(m:-1:2) r(1:Np-(m-1))];	 % build an auto-correlation matrix
     end                                         % a = lpc(x,Np), levinson(x,Np)
     a=-inv(R)*rr;                               % find coeffs of LPC filter
     gain=r(1)+r(2:Np+1)*a;                      % find filter gain
     H=freqz(1,[1;a]);                           % find filter frequency response
     if(ifigs==1) subplot(413); plot(abs(H)); title('Filter freq response'); end
   % compress=[compress; T; gain; a; ];          % store parameter values

   % SYNTHESIS - generate speech using calculated parameters
     T = 0; % remove "%" and set: T = 80, 50, 30, 0
     if (T~=0) where=where-Mstep; end            % next excitation=1 position
     for n=1:Mstep                               % SYNTHESIS LOOP START
         if( T==0)                               %
           % exc=2*(rand(1,1)-0.5); where=271;   % random excitation
             exc=0.5*randn(1,1); where=271;      % random excitation
         else                                    %
            if (n==where) exc=1; where=where+T;  % excitation = 1
            else exc=0; end                      % excitation = 0
         end                                     %
         ss(n) = gain*exc - bs*a;                % filtering excitation
         bs = [ss(n) bs(1:Np-1) ];               % shifting the output buffer
     end                                         % SYNTHESIS LOOP END
     s = [s ss];                                 % store the synthesized speech
     if(ifigs==1) subplot(414); plot(ss); title('Synthesized speech s(n)'); pause, end
end

% Finished!
figure; plot(s); title('Synthesized speech'); pause
soundsc(s,fs)
